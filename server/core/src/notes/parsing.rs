use crate::notes::content_schema::{
    CodeBlockData, ColumnType, Content, Element, ElementType, HeadingType, LinkData, LinkType,
    ListData, TableData, TaskListMarkerData,
};
use crate::API_VERSION;
use pulldown_cmark::{
    html, Alignment, CodeBlockKind, Event, LinkType as ParserLinkType, Options, Parser, Tag,
};
use std::ops::Range;

fn get_parser_options() -> Options {
    let mut parser_opts = Options::empty();
    parser_opts.insert(Options::ENABLE_TABLES);
    parser_opts.insert(Options::ENABLE_FOOTNOTES);
    parser_opts.insert(Options::ENABLE_STRIKETHROUGH);
    parser_opts.insert(Options::ENABLE_TASKLISTS);
    parser_opts
}

pub fn markdown_to_html(input: String) -> String {
    let parser = Parser::new_ext(input.as_str(), get_parser_options());

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

fn transform_link_type(link_type: ParserLinkType) -> LinkType {
    match link_type {
        ParserLinkType::Inline => LinkType::Inline,
        ParserLinkType::Reference => LinkType::Reference,
        ParserLinkType::ReferenceUnknown => LinkType::Reference,
        ParserLinkType::Collapsed => LinkType::Collapsed,
        ParserLinkType::CollapsedUnknown => LinkType::Collapsed,
        ParserLinkType::Shortcut => LinkType::Shortcut,
        ParserLinkType::ShortcutUnknown => LinkType::Shortcut,
        ParserLinkType::Autolink => LinkType::Autolink,
        ParserLinkType::Email => LinkType::Email,
    }
}

fn alignment_to_column_type(alignment: Alignment) -> ColumnType {
    match alignment {
        Alignment::None => ColumnType::None,
        Alignment::Left => ColumnType::Left,
        Alignment::Center => ColumnType::Center,
        Alignment::Right => ColumnType::Right,
    }
}

// TODO: refactor this mess
pub fn markdown_to_elements(content: String) -> Vec<Element> {
    let events: Vec<(Event, Range<usize>)> =
        Parser::new_ext(content.as_str(), get_parser_options())
            .into_offset_iter()
            .collect();
    let mut tag_stack: Vec<Tag> = Vec::new();
    let mut context: Vec<Vec<Element>> = Vec::new();
    let mut elements: Vec<Element> = Vec::new();

    for (event, _) in events {
        match event {
            Event::Start(tag) => {
                tag_stack.push(tag);
                context.push(elements.to_vec()); // TODO: avoid copying
                elements = Vec::new();
            }
            Event::End(end_tag) => {
                let start_tag = tag_stack.pop();
                if let Some(tag_) = start_tag {
                    if tag_ != end_tag {
                        error!("mismatched start & end tags");
                    }
                    let children = elements.to_vec(); // TODO: avoid copying
                    if let Some(context_) = context.pop() {
                        elements = context_.clone();
                    } else {
                        error!("corrupted parse context");
                        elements = Vec::new();
                    }
                    let element = Element {
                        element: match tag_ {
                            Tag::Paragraph => ElementType::Paragraph,
                            Tag::Heading(size) => ElementType::Heading(match size {
                                1 => HeadingType::H1,
                                2 => HeadingType::H2,
                                3 => HeadingType::H3,
                                4 => HeadingType::H4,
                                5 => HeadingType::H5,
                                _ => HeadingType::H6,
                            }),
                            Tag::BlockQuote => ElementType::BlockQuote,
                            Tag::CodeBlock(code_block_kind) => {
                                ElementType::CodeBlock(CodeBlockData {
                                    language: match code_block_kind {
                                        CodeBlockKind::Indented => None,
                                        CodeBlockKind::Fenced(lang) => Some(lang.into_string()),
                                    },
                                })
                            }
                            Tag::List(number_of_first_item) => ElementType::List(ListData {
                                number_of_first_item,
                            }),
                            Tag::Item => ElementType::Item,
                            Tag::FootnoteDefinition(label) => {
                                ElementType::FootnoteDefinition(label.into_string())
                            }
                            Tag::Table(alignments) => ElementType::Table(TableData {
                                column_types: alignments
                                    .iter()
                                    .map(|alignment| alignment_to_column_type(*alignment))
                                    .collect(),
                            }),
                            Tag::TableHead => ElementType::TableHead,
                            Tag::TableRow => ElementType::TableRow,
                            Tag::TableCell => ElementType::TableCell,
                            Tag::Emphasis => ElementType::Emphasis,
                            Tag::Strong => ElementType::Strong,
                            Tag::Strikethrough => ElementType::Strikethrough,
                            Tag::Link(link_type, destination_url, title) => {
                                ElementType::Link(LinkData {
                                    link_type: transform_link_type(link_type),
                                    destination_url: destination_url.into_string(),
                                    title: title.into_string(),
                                })
                            }
                            Tag::Image(link_type, destination_url, title) => {
                                ElementType::Image(LinkData {
                                    link_type: transform_link_type(link_type),
                                    destination_url: destination_url.into_string(),
                                    title: title.into_string(),
                                })
                            }
                        },
                        children: Some(children),
                    };
                    elements.push(element);
                } else {
                    error!("no start tag in stack");
                }
            }
            Event::Text(content) => {
                elements.push(Element {
                    element: ElementType::Text(content.into_string()),
                    children: None,
                });
            }
            Event::Code(content) => {
                elements.push(Element {
                    element: ElementType::Code(content.into_string()),
                    children: None,
                });
            }
            Event::Html(content) => {
                elements.push(Element {
                    element: ElementType::Html(content.into_string()),
                    children: None,
                });
            }
            Event::FootnoteReference(label) => {
                elements.push(Element {
                    element: ElementType::FootnoteReference(label.into_string()),
                    children: None,
                });
            }
            Event::SoftBreak => {
                elements.push(Element {
                    element: ElementType::SoftBreak,
                    children: None,
                });
            }
            Event::HardBreak => {
                elements.push(Element {
                    element: ElementType::HardBreak,
                    children: None,
                });
            }
            Event::Rule => {
                elements.push(Element {
                    element: ElementType::Rule,
                    children: None,
                });
            }
            Event::TaskListMarker(checked) => {
                elements.push(Element {
                    element: ElementType::TaskListMarker(TaskListMarkerData { checked }),
                    children: None,
                });
            }
        }
    }

    elements.to_vec() // TODO: avoid copying
}

pub fn parse_markdown_content(input: String) -> Content {
    Content {
        elements: markdown_to_elements(input),
        schema_version: API_VERSION.to_string(),
    }
}

#[cfg(test)]
pub mod markdown_parsing {
    use super::*;
    use crate::notes::content_schema::ElementType::*;
    use crate::notes::content_schema::HeadingType::*;
    use crate::notes::content_schema::LinkType::*;
    use crate::notes::content_schema::*;

    const TEST_URL: &str = "http://example.com";

    #[test]
    fn parse_to_html() {
        assert_eq!(
            markdown_to_html("# Hello world".to_string()),
            "<h1>Hello world</h1>\n"
        );
        assert_eq!(
            markdown_to_html("~~Hello world~~".to_string()),
            "<p><del>Hello world</del></p>\n"
        );
        assert_eq!(
            markdown_to_html("- [ ] hello\n- [ ] world".to_string()),
            "<ul>\n\
                <li><input disabled=\"\" type=\"checkbox\"/>\nhello</li>\n\
                <li><input disabled=\"\" type=\"checkbox\"/>\nworld</li>\n\
            </ul>\n"
        );
    }

    #[test]
    fn header_elements() {
        assert_eq!(
            markdown_to_elements("# Hello\n## world".to_string()),
            vec![
                Element {
                    element: Heading(H1),
                    children: Some(vec![Element {
                        element: Text("Hello".to_string()),
                        children: None
                    }])
                },
                Element {
                    element: Heading(H2),
                    children: Some(vec![Element {
                        element: Text("world".to_string()),
                        children: None
                    }])
                }
            ]
        );
    }

    #[test]
    fn strikethrough_elements() {
        assert_eq!(
            markdown_to_elements("~~Hello world~~".to_string()),
            vec![Element {
                element: Paragraph,
                children: Some(vec![Element {
                    element: Strikethrough,
                    children: Some(vec![Element {
                        element: Text("Hello world".to_string()),
                        children: None
                    }])
                }])
            }]
        );
    }

    #[test]
    fn task_list_elements() {
        assert_eq!(
            markdown_to_elements("- [ ] hello\n- [ ] world".to_string()),
            vec![Element {
                element: List(ListData {
                    number_of_first_item: None
                }),
                children: Some(vec![
                    Element {
                        element: Item,
                        children: Some(vec![
                            Element {
                                element: TaskListMarker(TaskListMarkerData { checked: false }),
                                children: None
                            },
                            Element {
                                element: Text("hello".to_string()),
                                children: None
                            }
                        ])
                    },
                    Element {
                        element: Item,
                        children: Some(vec![
                            Element {
                                element: TaskListMarker(TaskListMarkerData { checked: false }),
                                children: None
                            },
                            Element {
                                element: Text("world".to_string()),
                                children: None
                            }
                        ])
                    }
                ])
            }]
        );
    }

    #[test]
    fn link_elements() {
        assert_eq!(
            markdown_to_elements(format!("[hello]({} \"the title\")", TEST_URL).to_string()),
            vec![Element {
                element: Paragraph,
                children: Some(vec![Element {
                    element: Link(LinkData {
                        link_type: Inline,
                        destination_url: "http://example.com".to_string(),
                        title: "the title".to_string()
                    }),
                    children: Some(vec![Element {
                        element: Text("hello".to_string()),
                        children: None
                    }])
                }])
            }]
        );

        assert_eq!(
            markdown_to_elements(format!("<{}>", TEST_URL).to_string()),
            vec![Element {
                element: Paragraph,
                children: Some(vec![Element {
                    element: Link(LinkData {
                        link_type: Autolink,
                        destination_url: "http://example.com".to_string(),
                        title: "".to_string()
                    }),
                    children: Some(vec![Element {
                        element: Text("http://example.com".to_string()),
                        children: None
                    }])
                }])
            }]
        );
    }

    #[test]
    fn image_elements() {
        let image_url = format!("{}/image.jpg", TEST_URL);
        assert_eq!(
            markdown_to_elements(format!(
                "![test image]({} \"the title\")",
                image_url.to_string()
            )),
            vec![Element {
                element: Paragraph,
                children: Some(vec![Element {
                    element: Image(LinkData {
                        link_type: Inline,
                        destination_url: image_url.to_string(),
                        title: "the title".to_string()
                    }),
                    children: Some(vec![Element {
                        element: Text("test image".to_string()),
                        children: None
                    }])
                }])
            }]
        );
    }
}
