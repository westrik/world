use crate::notes::schema::*;

pub trait MarkdownRender {
    fn to_markdown(&self) -> String;
}

impl MarkdownRender for Content {
    fn to_markdown(&self) -> String {
        let event_strs: Vec<String> = self.elements.iter().map(|el| el.to_markdown()).collect();
        event_strs.join("")
    }
}

impl MarkdownRender for Element {
    fn to_markdown(&self) -> String {
        match &self.element {
            ElementType::Text(str) => str.to_string(),
            ElementType::Code(str) => format!("`{}`", str),
            ElementType::Html(_) => "".to_string(),
            ElementType::Paragraph => "".to_string(),
            ElementType::Emphasis => "".to_string(),
            ElementType::Strong => "".to_string(),
            ElementType::Strikethrough => "".to_string(),
            ElementType::Heading(heading_type) => "".to_string(),
            ElementType::Link(link_data) => "".to_string(),
            ElementType::Image(link_data) => "".to_string(),
            ElementType::CodeBlock(code_block_data) => "".to_string(),
            ElementType::List(list_data) => "".to_string(),
            ElementType::Item => "".to_string(),
            ElementType::TaskListMarker(marker_data) => "".to_string(),
            ElementType::BlockQuote => "".to_string(),
            ElementType::FootnoteDefinition(str) => "".to_string(),
            ElementType::FootnoteReference(str) => "".to_string(),
            ElementType::Table(table_data) => "".to_string(),
            ElementType::TableHead => "".to_string(),
            ElementType::TableRow => "".to_string(),
            ElementType::TableCell => "".to_string(),
            ElementType::SoftBreak => "".to_string(),
            ElementType::HardBreak => "".to_string(),
            ElementType::Rule => "".to_string(),
        }
    }
}

#[cfg(test)]
pub mod markdown_output {
    use super::*;
    use crate::notes::parsing::markdown_to_elements;
    use crate::notes::schema::Content;

    #[test]
    fn content_to_markdown() {
        let md = "- [ ] hello\n- [ ] world";
        let elements = markdown_to_elements(md.to_string());
        assert_eq!(
            Content {
                elements,
                schema_version: "v0.1.23".to_string()
            }
            .to_markdown(),
            md
        );
    }
}
