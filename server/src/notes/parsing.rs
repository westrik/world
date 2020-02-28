use pulldown_cmark::{html, Options, Parser};

#[derive(Deserialize, Serialize)]
pub struct Content {}

pub fn parse_markdown(input: String) -> Content {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input.as_str(), options);

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
    }
}
