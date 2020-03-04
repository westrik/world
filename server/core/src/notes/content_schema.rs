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
    CodeBlock(Option<String>),
    List(Option<u64>),
    Item,
    FootnoteDefinition(String),
    Table(Vec<ColumnType>),
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Link(LinkType, String, String),
    Image(LinkType, String, String),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum LinkType {
    // Inline link like `[foo](bar)`
    Inline,
    // Reference link like `[foo][bar]`
    Reference,
    // Collapsed link like `[foo][]`
    Collapsed,
    // Shortcut link like `[foo]`
    Shortcut,
    // Autolink like `<http://foo.bar/baz>`
    Autolink,
    // Email address in autolink like `<john@example.org>`
    Email,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ColumnType {
    None,
    Left,
    Center,
    Right,
}
