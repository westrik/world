use pulldown_cmark::{html, Event as ParserEvent, Options, Parser, Tag as ParserTag};

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    events: Vec<Event>,
}

impl Content {
    pub fn to_markdown(self) -> String {
        // TODO: actual output
        let event_strs: Vec<String> = self
            .events
            .iter()
            .map(|ev| {
                match ev {
                    // Event::Start(tag) => {},
                    // Event::End(_) => {},
                    // Event::Text(_) => {},
                    // Event::Code(_) => {},
                    // Event::Html(_) => {},
                    // Event::FootnoteReference(_) => {},
                    // Event::SoftBreak => {},
                    // Event::HardBreak => {},
                    // Event::Rule => {},
                    // Event::TaskListMarker(_) => {},
                    _ => ev.to_markdown(),
                }
            })
            .collect();
        event_strs.join("")
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Event {
    Start(Tag),
    End(Tag),
    Text(String),
    Code(String),
    Html(String),
    FootnoteReference(String),
    SoftBreak,
    HardBreak,
    Rule,
    TaskListMarker(bool),
}

impl Event {
    fn to_markdown(&self) -> String {
        "[Event]".to_string()
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Tag {
    Paragraph,
    Heading(u32),
    BlockQuote,
    CodeBlock, // TODO
    List(Option<u64>),
    Item,
    FootnoteDefinition(String),
    Table, // TODO
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Link,  // TODO
    Image, // TODO
}

impl Tag {
    fn _to_markdown(&self) -> String {
        "[Tag]".to_string()
    }
}

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

fn transform_parse_tag(tag: ParserTag) -> Tag {
    match tag {
        ParserTag::Paragraph => Tag::Paragraph,
        ParserTag::Heading(size) => Tag::Heading(size),
        ParserTag::BlockQuote => Tag::BlockQuote,
        ParserTag::CodeBlock(_) => Tag::CodeBlock,
        ParserTag::List(length) => Tag::List(length),
        ParserTag::Item => Tag::Item,
        ParserTag::FootnoteDefinition(fdef) => Tag::FootnoteDefinition(fdef.into_string()),
        ParserTag::Table(_) => Tag::Table,
        ParserTag::TableHead => Tag::TableHead,
        ParserTag::TableRow => Tag::TableRow,
        ParserTag::TableCell => Tag::TableCell,
        ParserTag::Emphasis => Tag::Emphasis,
        ParserTag::Strong => Tag::Strong,
        ParserTag::Strikethrough => Tag::Strikethrough,
        ParserTag::Link(_, _, _) => Tag::Link,
        ParserTag::Image(_, _, _) => Tag::Image,
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
        .map(|(event, _)| match event {
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
        })
        .collect()
}

#[cfg(test)]
pub mod test_resource_identifiers {
    use super::*;

    #[test]
    fn test_parsing() {
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
    fn test_events() {
        assert_eq!(
            markdown_to_event_list("# Hello world".to_string()),
            vec![
                Event::Start(Tag::Heading(1)),
                Event::Text("Hello world".to_string()),
                Event::End(Tag::Heading(1)),
            ]
        );
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
    fn test_content_to_markdown() {
        let md = "- [ ] hello\n- [ ] world";
        let events = markdown_to_event_list(md.to_string());
        assert_eq!(
            Content { events }.to_markdown(),
            "[Event][Event][Event][Event][Event]\
             [Event][Event][Event][Event][Event]" // TODO: fix this
        );
    }
}
