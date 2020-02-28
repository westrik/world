#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub events: Vec<Event>,
}

impl Content {
    pub fn to_markdown(&self) -> String {
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Event {
    Start(Tag),
    End(Tag),
    Text(String),
    Code(String),
    Html(String),
    FootnoteReference(String),
    SoftBreak,
    HardBreak,
    Rule,
    TaskListMarker(bool),
}

impl Event {
    fn to_markdown(&self) -> String {
        "[Event]".to_string()
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Tag {
    Paragraph,
    Heading(u32),
    BlockQuote,
    CodeBlock, // TODO
    List(Option<u64>),
    Item,
    FootnoteDefinition(String),
    Table, // TODO
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Link,  // TODO
    Image, // TODO
}

impl Tag {
    fn _to_markdown(&self) -> String {
        "[Tag]".to_string()
    }
}
