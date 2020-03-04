use crate::notes::content_schema::{ColumnType, Content, Event, LinkType, Tag};
use pulldown_cmark::{
    html, Alignment, CodeBlockKind, Event as ParserEvent, LinkType as ParserLinkType, Options,
    Parser, Tag as ParserTag,
};

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

fn transform_parse_event(event: ParserEvent) -> Event {
    match event {
        ParserEvent::Start(tag) => Event::Start(transform_parse_tag(tag)),
        ParserEvent::End(tag) => Event::End(transform_parse_tag(tag)),
        ParserEvent::Text(text) => Event::Text(text.into_string()),
        ParserEvent::Code(code) => Event::Code(code.into_string()),
        ParserEvent::Html(html) => Event::Html(html.into_string()),
        ParserEvent::FootnoteReference(fref) => Event::FootnoteReference(fref.into_string()),
        ParserEvent::SoftBreak => Event::SoftBreak,
        ParserEvent::HardBreak => Event::HardBreak,
        ParserEvent::Rule => Event::Rule,
        ParserEvent::TaskListMarker(status) => Event::TaskListMarker(status),
    }
}

fn transform_parse_tag(tag: ParserTag) -> Tag {
    match tag {
        ParserTag::Paragraph => Tag::Paragraph,
        ParserTag::Heading(size) => Tag::Heading(size),
        ParserTag::BlockQuote => Tag::BlockQuote,
        ParserTag::CodeBlock(kind) => match kind {
            CodeBlockKind::Indented => Tag::CodeBlock(None),
            CodeBlockKind::Fenced(lang) => Tag::CodeBlock(Some(lang.into_string())),
        },
        ParserTag::List(length) => Tag::List(length),
        ParserTag::Item => Tag::Item,
        ParserTag::FootnoteDefinition(fdef) => Tag::FootnoteDefinition(fdef.into_string()),
        ParserTag::Table(alignments) => Tag::Table(
            alignments
                .iter()
                .map(|alignment| match *alignment {
                    Alignment::None => ColumnType::None,
                    Alignment::Left => ColumnType::Left,
                    Alignment::Center => ColumnType::Center,
                    Alignment::Right => ColumnType::Right,
                })
                .collect(),
        ),
        ParserTag::TableHead => Tag::TableHead,
        ParserTag::TableRow => Tag::TableRow,
        ParserTag::TableCell => Tag::TableCell,
        ParserTag::Emphasis => Tag::Emphasis,
        ParserTag::Strong => Tag::Strong,
        ParserTag::Strikethrough => Tag::Strikethrough,
        ParserTag::Link(link_type, dest_url, title) => Tag::Link(
            transform_link_type(link_type),
            dest_url.into_string(),
            title.into_string(),
        ),
        ParserTag::Image(link_type, dest_url, title) => Tag::Image(
            transform_link_type(link_type),
            dest_url.into_string(),
            title.into_string(),
        ),
    }
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

pub fn _print_event_list_for_markdown(input: String) {
    let offset_iter = Parser::new_ext(input.as_str(), get_parser_options()).into_offset_iter();

    for (event, range) in offset_iter {
        println!("{:?}, {:?}", event, range);
    }
}

pub fn markdown_to_event_list(input: String) -> Vec<Event> {
    Parser::new_ext(input.as_str(), get_parser_options())
        .into_offset_iter()
        .map(|(event, _)| transform_parse_event(event))
        .collect()
}

pub fn parse_markdown_content(input: String) -> Content {
    Content {
        events: markdown_to_event_list(input),
    }
}

#[cfg(test)]
pub mod markdown_parsing {
    use super::*;
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
    fn header_events() {
        assert_eq!(
            markdown_to_event_list("# Hello\n## world".to_string()),
            vec![
                Event::Start(Tag::Heading(1)),
                Event::Text("Hello".to_string()),
                Event::End(Tag::Heading(1)),
                Event::Start(Tag::Heading(2)),
                Event::Text("world".to_string()),
                Event::End(Tag::Heading(2)),
            ]
        );
    }

    #[test]
    fn strikethrough_events() {
        assert_eq!(
            markdown_to_event_list("~~Hello world~~".to_string()),
            vec![
                Event::Start(Tag::Paragraph),
                Event::Start(Tag::Strikethrough),
                Event::Text("Hello world".to_string()),
                Event::End(Tag::Strikethrough),
                Event::End(Tag::Paragraph),
            ]
        );
    }

    #[test]
    fn task_list_events() {
        assert_eq!(
            markdown_to_event_list("- [ ] hello\n- [ ] world".to_string()),
            vec![
                Event::Start(Tag::List(None)),
                Event::Start(Tag::Item),
                Event::TaskListMarker(false),
                Event::Text("hello".to_string()),
                Event::End(Tag::Item),
                Event::Start(Tag::Item),
                Event::TaskListMarker(false),
                Event::Text("world".to_string()),
                Event::End(Tag::Item),
                Event::End(Tag::List(None)),
            ]
        );
    }

    #[test]
    fn link_events() {
        assert_eq!(
            markdown_to_event_list(format!("[hello]({} \"the title\")", TEST_URL).to_string()),
            vec![
                Event::Start(Tag::Paragraph),
                Event::Start(Tag::Link(
                    LinkType::Inline,
                    TEST_URL.to_string(),
                    "the title".to_string()
                )),
                Event::Text("hello".to_string()),
                Event::End(Tag::Link(
                    LinkType::Inline,
                    TEST_URL.to_string(),
                    "the title".to_string()
                )),
                Event::End(Tag::Paragraph),
            ]
        );

        assert_eq!(
            markdown_to_event_list(format!("<{}>", TEST_URL).to_string()),
            vec![
                Event::Start(Tag::Paragraph),
                Event::Start(Tag::Link(
                    LinkType::Autolink,
                    TEST_URL.to_string(),
                    "".to_string()
                )),
                Event::Text(TEST_URL.to_string()),
                Event::End(Tag::Link(
                    LinkType::Autolink,
                    TEST_URL.to_string(),
                    "".to_string()
                )),
                Event::End(Tag::Paragraph),
            ]
        );
    }

    #[test]
    fn image_events() {
        let image_url = format!("{}/image.jpg", TEST_URL);
        assert_eq!(
            markdown_to_event_list(format!(
                "![test image]({} \"the title\")",
                image_url.to_string()
            )),
            vec![
                Event::Start(Tag::Paragraph),
                Event::Start(Tag::Image(
                    LinkType::Inline,
                    image_url.to_string(),
                    "the title".to_string()
                )),
                Event::Text("test image".to_string()),
                Event::End(Tag::Image(
                    LinkType::Inline,
                    image_url.to_string(),
                    "the title".to_string()
                )),
                Event::End(Tag::Paragraph),
            ]
        );
    }
}
