use crate::notes::content_schema::*;

pub trait MarkdownRender {
    fn to_markdown(&self) -> String;
}

impl MarkdownRender for Content {
    fn to_markdown(&self) -> String {
        // TODO: actual output
        let event_strs: Vec<String> = self
            .elements
            .iter()
            .map(|el| {
                match el {
                    // Event::Start(tag) => {},
                    // Event::End(_) => {},
                    // Event::Text(_) => {},
                    // Event::Code(_) => {},
                    // Event::Html(_) => {},
                    // Event::FootnoteReference(_) => {},
                    // Event::SoftBreak => {},
                    // Event::HardBreak => {},
                    // Event::Rule => {},
                    // Event::TaskListMarker(_) => {},
                    _ => el.to_markdown(),
                }
            })
            .collect();
        event_strs.join("")
    }
}

impl MarkdownRender for Element {
    fn to_markdown(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
pub mod test_markdown_output {
    // use crate::notes::content_schema::Content;
    use crate::notes::parsing::markdown_to_elements;

    #[test]
    fn test_content_to_markdown() {
        let md = "- [ ] hello\n- [ ] world";
        let _elements = markdown_to_elements(md.to_string());
        // assert_eq!(
        //     Content { elements }.to_markdown(),
        //     "[Event][Event][Event][Event][Event]\
        //          [Event][Event][Event][Event][Event]" // TODO: fix this
        // );
    }
}
