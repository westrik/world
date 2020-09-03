use std::ops::Deref;

use crate::notes::schema::{ElementType, HeadingType};
use crate::notes::{
    export::Render,
    schema::{Content, Element},
};

pub struct Html {
    output: String,
}

impl Deref for Html {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.output
    }
}

impl Render<Html> for Option<Vec<Element>> {
    fn render(&self) -> Html {
        match self {
            Some(list) => list.render(),
            None => Html {
                output: "".to_string(),
            },
        }
    }
}

impl Render<Html> for Vec<Element> {
    fn render(&self) -> Html {
        let elements_str = self
            .iter()
            .map(move |el| {
                let html: Html = el.render();
                html.output
            })
            .collect::<Vec<String>>();
        Html {
            output: elements_str.join(""),
        }
    }
}

impl Render<Html> for Content {
    fn render(&self) -> Html {
        self.elements.render()
    }
}

impl Render<Html> for Element {
    fn render(&self) -> Html {
        match &self.element {
            ElementType::Text(str) => Html {
                output: str.to_string(),
            },
            ElementType::Code(str) => Html {
                output: format!("<code>{}</code>", str),
            },
            ElementType::Html(_) => Html {
                output: "".to_string(),
            },
            ElementType::Paragraph => Html {
                output: format!("<p>{}</p>", *self.children.render()),
            },
            ElementType::Emphasis => Html {
                output: format!("<em>{}</em>", *self.children.render()),
            },
            ElementType::Strong => Html {
                output: format!("<strong>{}</strong>", *self.children.render()),
            },
            ElementType::Strikethrough => Html {
                output: format!("<strike>{}</strike>", *self.children.render()),
            },
            ElementType::Heading(heading_type) => {
                let tag = match heading_type {
                    HeadingType::H1 => "h1",
                    HeadingType::H2 => "h2",
                    HeadingType::H3 => "h3",
                    HeadingType::H4 => "h4",
                    HeadingType::H5 => "h5",
                    HeadingType::H6 => "h6",
                };
                Html {
                    output: format!("<{}>{}<{}>", tag, *self.children.render(), tag),
                }
            }
            ElementType::Link(link_data) => Html {
                // TODO: handle emails, etc?
                output: format!(
                    "<a href=\"{}\">{}</a>",
                    link_data.destination_url, link_data.title
                ),
            },
            ElementType::Image(link_data) => Html {
                output: format!(
                    "<img src=\"{}\" alt=\"{}\" />",
                    link_data.destination_url, link_data.title
                ),
            },
            ElementType::CodeBlock(_code_block_data) => Html {
                // TODO: use code_block_data
                // TODO: render with syntect?
                output: format!("<pre>{}</pre>", *self.children.render()),
            },
            ElementType::List(_list_data) => Html {
                // TODO: use list_data
                output: format!("<ul>{}</ul>", *self.children.render()),
            },
            ElementType::Item => Html {
                output: format!("<li>{}</li>", *self.children.render()),
            },
            ElementType::TaskListMarker(marker_data) => Html {
                output: format!(
                    "<input type=\"checkbox\" {}/>",
                    match marker_data.checked {
                        true => "checked=\"checked\" ",
                        false => "",
                    }
                ),
            },
            ElementType::BlockQuote => Html {
                output: format!("<blockquote>{}</blockquote>", *self.children.render()),
            },
            ElementType::FootnoteDefinition(_str) => Html {
                // TODO: use str
                output: "".to_string(),
            },
            ElementType::FootnoteReference(footnote_ref) => Html {
                output: format!(
                    "<sup class=\"footnote-reference\"><a href=\"#{}\">{}</a></sup>",
                    footnote_ref, footnote_ref
                ),
            },
            ElementType::Table(_table_data) => Html {
                // TODO: use table_data
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
                output: "<wbr />".to_string(),
            },
            ElementType::HardBreak => Html {
                output: "<br />".to_string(),
            },
            ElementType::Rule => Html {
                output: "<hr />".to_string(),
            },
        }
    }
}

#[cfg(test)]
pub mod html_output {
    use super::*;
    use crate::notes::parsing::markdown_to_elements;
    use crate::notes::schema::Content;

    #[test]
    fn test_render_content_to_html() {
        let md = "- [ ] hello\n- [ ] world";
        let elements = markdown_to_elements(md.to_string());

        let rendered: Html = Content {
            elements,
            schema_version: "v0.1.23".to_string(),
        }
        .render();
        assert_eq!(
            *rendered,
            r#"<ul><li><input type="checkbox" />hello</li><li><input type="checkbox" />world</li></ul>"#,
        );
    }
}
