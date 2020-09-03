use crate::notes::export::Render;
use crate::notes::schema::*;

pub struct Markdown {
    output: String,
}

impl Render<Markdown> for Content {
    fn render(&self) -> Markdown {
        let event_strs: Vec<String> = self
            .elements
            .iter()
            .map(|el| {
                let md: Markdown = el.render();
                md.output
            })
            .collect();
        Markdown {
            output: event_strs.join(""),
        }
    }
}

impl Render<Markdown> for Element {
    fn render(&self) -> Markdown {
        match &self.element {
            // ElementType::Text(str) => Markdown {
            //     output: str.to_string(),
            // },
            // ElementType::Code(str) => Markdown {
            //     output: format!("`{}`", str),
            // },
            // ElementType::Html(_) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Paragraph => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Emphasis => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Strong => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Strikethrough => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Heading(heading_type) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Link(link_data) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Image(link_data) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::CodeBlock(code_block_data) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::List(list_data) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Item => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::TaskListMarker(marker_data) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::BlockQuote => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::FootnoteDefinition(str) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::FootnoteReference(str) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Table(table_data) => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::TableHead => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::TableRow => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::TableCell => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::SoftBreak => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::HardBreak => Markdown {
            //     output: "".to_string(),
            // },
            // ElementType::Rule => Markdown {
            //     output: "".to_string(),
            // },
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
pub mod markdown_output {
    use super::*;
    use crate::notes::parsing::markdown_to_elements;
    use crate::notes::schema::Content;

    #[test]
    fn content_markdown_render() {
        let md = "- [ ] hello\n- [ ] world";
        let elements = markdown_to_elements(md.to_string());

        let rendered: Markdown = Content {
            elements,
            schema_version: "v0.1.23".to_string(),
        }
        .render();
        assert_eq!(rendered.output, md);
    }
}
