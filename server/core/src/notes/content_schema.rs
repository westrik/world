#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub events: Vec<Event>,
}

// mirrors https://docs.rs/pulldown-cmark/0.7.0/pulldown_cmark/enum.Event.html
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "event")]
pub enum Event {
    #[serde(rename = "start")]
    Start(Tag),
    #[serde(rename = "end")]
    End(Tag),
    #[serde(rename = "text")]
    Text(String),
    #[serde(rename = "code")]
    Code(String),
    #[serde(rename = "html")]
    Html(String),
    #[serde(rename = "footnoteReference")]
    FootnoteReference(String),
    #[serde(rename = "softBreak")]
    SoftBreak,
    #[serde(rename = "hardBreak")]
    HardBreak,
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "taskListMarker")]
    TaskListMarker(bool),
}

// mirrors https://docs.rs/pulldown-cmark/0.7.0/pulldown_cmark/enum.Tag.html
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "tag")]
pub enum Tag {
    #[serde(rename = "paragraph")]
    Paragraph,
    #[serde(rename = "heading")]
    Heading(u32),
    #[serde(rename = "blockquote")]
    BlockQuote,
    #[serde(rename = "codeBlock")]
    CodeBlock(Option<String>),
    #[serde(rename = "list")]
    List(Option<u64>),
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "footnoteDefinition")]
    FootnoteDefinition(String),
    #[serde(rename = "table")]
    Table(Vec<ColumnType>),
    #[serde(rename = "tableHead")]
    TableHead,
    #[serde(rename = "tableRow")]
    TableRow,
    #[serde(rename = "tableCell")]
    TableCell,
    #[serde(rename = "emphasis")]
    Emphasis,
    #[serde(rename = "strong")]
    Strong,
    #[serde(rename = "strikethrough")]
    Strikethrough,
    #[serde(rename = "link")]
    Link(LinkType, String, String),
    #[serde(rename = "image")]
    Image(LinkType, String, String),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "linkType")]
pub enum LinkType {
    // Inline link like `[foo](bar)`
    #[serde(rename = "inline")]
    Inline,
    // Reference link like `[foo][bar]`
    #[serde(rename = "reference")]
    Reference,
    // Collapsed link like `[foo][]`
    #[serde(rename = "collapsed")]
    Collapsed,
    // Shortcut link like `[foo]`
    #[serde(rename = "shortcut")]
    Shortcut,
    // Autolink like `<http://foo.bar/baz>`
    #[serde(rename = "autolink")]
    Autolink,
    // Email address in autolink like `<john@example.org>`
    #[serde(rename = "email")]
    Email,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "columnType")]
pub enum ColumnType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right,
}
