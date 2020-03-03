use crate::notes::content_schema::*;

pub trait MarkdownRender {
    fn to_markdown(&self) -> String;
}

impl MarkdownRender for Content {
    fn to_markdown(&self) -> String {
        // TODO: actual output
        let event_strs: Vec<String> = self
            .events
            .iter()
            .map(|ev| {
                match ev {
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
                    _ => ev.to_markdown(),
                }
            })
            .collect();
        event_strs.join("")
    }
}

impl MarkdownRender for Event {
    fn to_markdown(&self) -> String {
        "[Event]".to_string()
    }
}

impl MarkdownRender for Tag {
    fn to_markdown(&self) -> String {
        "[Tag]".to_string()
    }
}

#[cfg(test)]
pub mod test_markdown_output {
    use crate::notes::content_schema::Content;
    use crate::notes::output::MarkdownRender;
    use crate::notes::parsing::markdown_to_event_list;

    #[test]
    fn test_content_to_markdown() {
        let md = "- [ ] hello\n- [ ] world";
        let events = markdown_to_event_list(md.to_string());
        assert_eq!(
            Content { events }.to_markdown(),
            "[Event][Event][Event][Event][Event]\
                 [Event][Event][Event][Event][Event]" // TODO: fix this
        );
    }
}
