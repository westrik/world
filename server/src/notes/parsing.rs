use pulldown_cmark::{html, Options, Parser};

#[derive(Deserialize, Serialize)]
pub struct Content {}

pub fn parse_markdown(input: String) -> Content {
    let mut parser_opts = Options::empty();
    parser_opts.insert(Options::ENABLE_TABLES);
    parser_opts.insert(Options::ENABLE_FOOTNOTES);
    parser_opts.insert(Options::ENABLE_STRIKETHROUGH);
    parser_opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(input.as_str(), parser_opts);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("{}", html_output);

    Content {}
}

#[cfg(test)]
pub mod test_resource_identifiers {
    use super::*;

    #[test]
    fn test_parsing() {
        parse_markdown("# Hello world".to_string());
        parse_markdown("~~Hello world~~".to_string());
        parse_markdown("- [ ] hello\n- [ ] world".to_string());
    }
}
