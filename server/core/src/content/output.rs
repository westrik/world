use crate::content::schema::*;

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
pub mod markdown_output {
    // use crate::content::schema::Content;
    use crate::content::parsing::markdown_to_elements;

    #[test]
    fn content_to_markdown() {
        let md = "- [ ] hello\n- [ ] world";
        let _elements = markdown_to_elements(md.to_string());
        // assert_eq!(
        //     Content { elements }.to_markdown(),
        //     "[Event][Event][Event][Event][Event]\
        //          [Event][Event][Event][Event][Event]" // TODO: fix this
        // );
    }
}
