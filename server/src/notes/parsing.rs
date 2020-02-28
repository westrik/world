use pulldown_cmark::{html, Options, Parser};

#[derive(Deserialize, Serialize)]
pub struct Content {
    events: Vec<Event>,
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
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
}
