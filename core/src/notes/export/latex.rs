use crate::notes::{
    export::Render,
    schema::{Content, Element},
};

pub struct Latex {
    output: String,
}

impl Render<Latex> for Content {
    fn render(&self) -> Latex {
        let event_strs: Vec<String> = self
            .elements
            .iter()
            .map(|el| {
                let latex: Latex = el.render();
                latex.output
            })
            .collect();
        Latex {
            output: event_strs.join(""),
        }
    }
}

impl Render<Latex> for Element {
    fn render(&self) -> Latex {
        match &self.element {
            // ElementType::Text(str) => Latex {
            //     output: str.to_string(),
            // },
            // ElementType::Code(str) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Html(_) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Paragraph => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Emphasis => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Strong => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Strikethrough => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Heading(heading_type) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Link(link_data) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Image(link_data) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::CodeBlock(code_block_data) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::List(list_data) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Item => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::TaskListMarker(marker_data) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::BlockQuote => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::FootnoteDefinition(str) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::FootnoteReference(str) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Table(table_data) => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::TableHead => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::TableRow => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::TableCell => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::SoftBreak => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::HardBreak => Latex {
            //     output: "".to_string(),
            // },
            // ElementType::Rule => Latex {
            //     output: "".to_string(),
            // },
            _ => unimplemented!(),
        }
    }
}
