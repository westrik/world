use std::ops::Deref;

use crate::notes::schema::{ElementType, HeadingType, LinkType};
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
            ElementType::Link(link_data) => {
                let url_prefix = match link_data.link_type {
                    LinkType::Email => "mailto:",
                    _ => "",
                };
                Html {
                    output: format!(
                        "<a href=\"{}{}\">{}</a>",
                        url_prefix,
                        link_data.destination_url,
                        *self.children.render()
                    ),
                }
            }
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
            ElementType::List(list_data) => {
                let output = match list_data.number_of_first_item {
                    Some(first_item_idx) => format!(
                        "<ol start=\"{}\">{}</ol>",
                        first_item_idx,
                        *self.children.render()
                    ),
                    None => format!("<ul>{}</ul>", *self.children.render()),
                };
                Html { output }
            }
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
            ElementType::FootnoteDefinition(str) => Html {
                output: format!(
                    r#"<div class="footnote-definition" id="{}"><sup class="footnote-definition-label">{}</sup>{}</div>"#,
                    str,
                    str,
                    *self.children.render()
                ),
            },
            ElementType::FootnoteReference(footnote_ref) => Html {
                output: format!(
                    "<sup class=\"footnote-reference\"><a href=\"#{}\">{}</a></sup>",
                    footnote_ref, footnote_ref
                ),
            },
            ElementType::Table(_table_data) => Html {
                // TODO: use table_data
                output: format!("<table>{}</table>", *self.children.render()),
            },
            ElementType::TableHead => Html {
                output: format!("<thead>{}</thead>", *self.children.render()),
            },
            ElementType::TableRow => Html {
                output: format!("<tr>{}</tr>", *self.children.render()),
            },
            ElementType::TableCell => Html {
                output: format!("<td>{}</td>", *self.children.render()),
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
Hello this is a ref[^ref_text] and another[^1] and another one [^link_text]
[ref_text]: https://www.example.org
[1]: http://example.org
[link_text]: http://www.example.com
"#,
            r##"<p>Hello this is a ref<sup class="footnote-reference"><a href="#ref_text">ref_text</a></sup> and another<sup class="footnote-reference"><a href="#1">1</a></sup> and another one <sup class="footnote-reference"><a href="#link_text">link_text</a></sup><wbr />[ref_text]: https://www.example.org<wbr />[1]: http://example.org<wbr />[link_text]: http://www.example.com</p>"##
        );
    }

    #[test]
    fn test_table() {
        assert_md_to_html!(
            r#"
Colons can be used to align columns.

| Tables        | Are           | Cool  |
| ------------- |:-------------:| -----:|
| col 3 is      | right-aligned | $1600 |
| col 2 is      | centered      |   $12 |
| zebra stripes | are neat      |    $1 |

There must be at least 3 dashes separating each header cell.
The outer pipes (|) are optional, and you don't need to make the
raw Markdown line up prettily. You can also use inline Markdown.

Markdown | Less | Pretty
--- | --- | ---
*Still* | `renders` | **nicely**
1 | 2 | 3
"#,
            r#"<p>Colons can be used to align columns.</p><table><thead><td>Tables</td><td>Are</td><td>Cool</td></thead><tr><td>col 3 is</td><td>right-aligned</td><td>$1600</td></tr><tr><td>col 2 is</td><td>centered</td><td>$12</td></tr><tr><td>zebra stripes</td><td>are neat</td><td>$1</td></tr></table><p>There must be at least 3 dashes separating each header cell.<wbr />The outer pipes (|) are optional, and you don't need to make the<wbr />raw Markdown line up prettily. You can also use inline Markdown.</p><table><thead><td>Markdown</td><td>Less</td><td>Pretty</td></thead><tr><td><em>Still</em></td><td><code>renders</code></td><td><strong>nicely</strong></td></tr><tr><td>1</td><td>2</td><td>3</td></tr></table>"#
        );
    }

    #[test]
    fn test_blockquote() {
        assert_md_to_html!(
            "> hello world",
            "<blockquote><p>hello world</p></blockquote>"
        );
    }

    #[test]
    fn test_inline_code() {
        assert_md_to_html!("`hello {world};`", "<p><code>hello {world};</code></p>");
    }

    #[test]
    fn test_code_block() {
        assert_md_to_html!(
            r#"```sh
function hello_world() {
    echo "hello";
}
```"#,
            r#"<pre>function hello_world() {
    echo "hello";
}
</pre>"#
        );
    }

    #[test]
    fn test_unordered_list() {
        assert_md_to_html!(
            r#"
- hello
- world
"#,
            "<ul><li>hello</li><li>world</li></ul>"
        );
    }

    #[test]
    fn test_offset_ordered_list() {
        assert_md_to_html!(
            r#"
3. hello
4. world
5. hello world!
        "#,
            r#"<ol start="3"><li>hello</li><li>world</li><li>hello world!</li></ol>"#
        );
    }

    #[test]
    fn test_task_list() {
        assert_md_to_html!(
            "- [ ] hello\n- [ ] world",
            r#"<ul><li><input type="checkbox" />hello</li><li><input type="checkbox" />world</li></ul>"#
        );
    }

    #[test]
    fn test_rules() {
        assert_md_to_html!(
            r#"------------------
hello\
world
in the world!
"#,
            "<hr /><p>hello<br />world<wbr />in the world!</p>"
        );
    }
}
