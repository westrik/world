#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub elements: Vec<Element>,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Element {
    pub element: ElementType,
    pub children: Option<Vec<Element>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "elementType")]
pub enum ElementType {
    #[serde(rename = "text")]
    Text(String),
    #[serde(rename = "code")]
    Code(String),
    #[serde(rename = "html")]
    Html(String),
    #[serde(rename = "p")]
    Paragraph,
    #[serde(rename = "em")]
    Emphasis,
    #[serde(rename = "strong")]
    Strong,
    #[serde(rename = "strike")]
    Strikethrough,
    #[serde(rename = "header")]
    Heading(HeadingType),
    #[serde(rename = "link")]
    Link(LinkData),
    #[serde(rename = "image")]
    Image(LinkData),
    #[serde(rename = "codeBlock")]
    CodeBlock(CodeBlockData),
    #[serde(rename = "list")]
    List(ListData),
    #[serde(rename = "listItem")]
    Item,
    #[serde(rename = "taskListMarker")]
    TaskListMarker(TaskListMarkerData),
    #[serde(rename = "blockQuote")]
    BlockQuote,
    #[serde(rename = "footnoteDefinition")]
    FootnoteDefinition(String),
    #[serde(rename = "footnoteReference")]
    FootnoteReference(String),
    #[serde(rename = "table")]
    Table(TableData),
    #[serde(rename = "tableHead")]
    TableHead,
    #[serde(rename = "tableRow")]
    TableRow,
    #[serde(rename = "tableCell")]
    TableCell,
    #[serde(rename = "softBreak")]
    SoftBreak,
    #[serde(rename = "hardBreak")]
    HardBreak,
    #[serde(rename = "rule")]
    Rule,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum HeadingType {
    #[serde(rename = "h1")]
    H1,
    #[serde(rename = "h2")]
    H2,
    #[serde(rename = "h3")]
    H3,
    #[serde(rename = "h4")]
    H4,
    #[serde(rename = "h5")]
    H5,
    #[serde(rename = "h6")]
    H6,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "linkData")]
pub struct LinkData {
    #[serde(rename = "type")]
    pub link_type: LinkType,
    #[serde(rename = "destinationUrl")]
    pub destination_url: String,
    pub title: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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
pub struct CodeBlockData {
    pub language: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ListData {
    #[serde(rename = "numberOfFirstItem")]
    pub number_of_first_item: Option<u64>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TaskListMarkerData {
    pub checked: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "tableData")]
pub struct TableData {
    #[serde(rename = "columnTypes")]
    pub column_types: Vec<ColumnType>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "alignment")]
pub enum ColumnType {
    #[serde(rename = "none")]
    Unaligned,
    #[serde(rename = "left")]
    LeftAligned,
    #[serde(rename = "center")]
    CenterAligned,
    #[serde(rename = "right")]
    RightAligned,
}
