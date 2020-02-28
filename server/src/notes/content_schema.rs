#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub events: Vec<Event>,
}

// mirrors https://docs.rs/pulldown-cmark/0.7.0/pulldown_cmark/enum.Event.html
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

// mirrors https://docs.rs/pulldown-cmark/0.7.0/pulldown_cmark/enum.Tag.html
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
