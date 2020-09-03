use crate::notes::{
    export::Render,
    schema::{Content, Element, ElementType},
};

pub struct Html {
    output: String,
}

impl Render<Html> for Content {
    fn render(&self) -> Html {
        let event_strs: Vec<String> = self
            .elements
            .iter()
            .map(|el| {
                let html: Html = el.render();
                html.output
            })
            .collect();
        Html {
            output: event_strs.join(""),
        }
    }
}

impl Render<Html> for Element {
    fn render(&self) -> Html {
        match &self.element {
            ElementType::Text(str) => Html {
                output: str.to_string(),
            },
            ElementType::Code(str) => Html {
                output: "".to_string(),
            },
            ElementType::Html(_) => Html {
                output: "".to_string(),
            },
            ElementType::Paragraph => Html {
                output: "".to_string(),
            },
            ElementType::Emphasis => Html {
                output: "".to_string(),
            },
            ElementType::Strong => Html {
                output: "".to_string(),
            },
            ElementType::Strikethrough => Html {
                output: "".to_string(),
            },
            ElementType::Heading(heading_type) => Html {
                output: "".to_string(),
            },
            ElementType::Link(link_data) => Html {
                output: "".to_string(),
            },
            ElementType::Image(link_data) => Html {
                output: "".to_string(),
            },
            ElementType::CodeBlock(code_block_data) => Html {
                output: "".to_string(),
            },
            ElementType::List(list_data) => Html {
                output: "".to_string(),
            },
            ElementType::Item => Html {
                output: "".to_string(),
            },
            ElementType::TaskListMarker(marker_data) => Html {
                output: "".to_string(),
            },
            ElementType::BlockQuote => Html {
                output: "".to_string(),
            },
            ElementType::FootnoteDefinition(str) => Html {
                output: "".to_string(),
            },
            ElementType::FootnoteReference(str) => Html {
                output: "".to_string(),
            },
            ElementType::Table(table_data) => Html {
                output: "".to_string(),
            },
            ElementType::TableHead => Html {
                output: "".to_string(),
            },
            ElementType::TableRow => Html {
                output: "".to_string(),
            },
            ElementType::TableCell => Html {
                output: "".to_string(),
            },
            ElementType::SoftBreak => Html {
                output: "".to_string(),
            },
            ElementType::HardBreak => Html {
                output: "".to_string(),
            },
            ElementType::Rule => Html {
                output: "".to_string(),
            },
        }
    }
}
