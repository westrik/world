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
                    output: format!("<{}>{}</{}>", tag, *self.children.render(), tag),
                }
            }
            ElementType::Link(link_data) => Html {
                // TODO: handle emails, etc?
                output: format!(
                    "<a href=\"{}\">{}</a>",
                    link_data.destination_url,
                    *self.children.render()
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
    use crate::API_VERSION;

    macro_rules! assert_md_to_html {
        ($md:expr, $html:expr) => {
            let elements = markdown_to_elements($md.to_string());
            let rendered: Html = Content {
                elements,
                schema_version: API_VERSION.to_string(),
            }
            .render();
            assert_eq!(*rendered, $html);
        };
    }

    #[test]
    fn test_text() {
        assert_md_to_html!(
            "**bold**  _italic_ ~~strikethrough!~~",
            r#"<p><strong>bold</strong>  <em>italic</em> <strike>strikethrough!</strike></p>"#
        );
    }

    #[test]
    fn test_headers() {
        assert_md_to_html!(r#"
# H1
## H2
### H3
#### H4
##### H5
###### H6

Alt-H1
======

Alt-H2
------

"#,
            "<h1>H1</h1><h2>H2</h2><h3>H3</h3><h4>H4</h4><h5>H5</h5><h6>H6</h6><h1>Alt-H1</h1><h2>Alt-H2</h2>"
        );
    }

    #[test]
    fn test_links() {
        assert_md_to_html!(
            r#"
[example](https://example.com)

[example](https://example.com "example!")

[example](../example)

text and [example]. send an email to <me@example.com>.

http://www.example.com and <http://www.example.com> and example.com.
        "#,
            r#"<p><a href="https://example.com">example</a></p><p><a href="https://example.com">example</a></p><p><a href="../example">example</a></p><p>text and [example]. send an email to <a href="mailto:me@example.com">me@example.com</a>.</p><p>http://www.example.com and <a href="http://www.example.com">http://www.example.com</a> and example.com.</p>"#
        );
    }

    #[test]
    fn test_images() {
        assert_md_to_html!(
            r#"[![Test image with link](/test-image.png "test image description")](https://example.com/image.png)"#,
            r#"<p><a href="https://example.com/image.png"><img src="/test-image.png" alt="test image description" /></a></p>"#
        );
    }

    #[test]
    fn test_footnotes() {
        assert_md_to_html!(
            r#"
[ref text]: https://www.example.org
[1]: http://example.org
[link text]: http://www.example.com
"#,
            r#""#
        );
    }

    // TODO: tables
    // TODO: blockquotes
    // TODO: inline code
    // TODO: code blocks
    // TODO: numbered lists (+ lists that start after 1)

    #[test]
    fn test_task_list() {
        assert_md_to_html!(
            "- [ ] hello\n- [ ] world",
            r#"<ul><li><input type="checkbox" />hello</li><li><input type="checkbox" />world</li></ul>"#
        );
    }
}
