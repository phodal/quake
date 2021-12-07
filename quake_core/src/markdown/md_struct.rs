#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MdStruct {
    Br,
    Blockquote(String),
    /// is_checked
    Checkbox {
        is_checked: bool,
    },
    Code(MdCode),
    CodeSpan(String),
    Del(String),
    Em(String),
    Heading(MdHeading),
    Hr,
    Html(String),
    Image(MdImage),
    Link(MdLink),
    List(MdList),
    ListItem(MdListItem),
    Paragraph(String),
    Strong(String),
    Table(MdTableRow),
    TabelCell(MdTableCell),
    TableRow(MdTableRow),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MdCode {
    code: String,
    language: Option<String>,
    is_escaped: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MdHeading {
    text: String,
    level: i8,
    raw: String,
    slugger: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MdImage {
    href: Option<String>,
    title: Option<String>,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MdLink {
    href: Option<String>,
    title: Option<String>,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MdList {
    body: String,
    ordered: bool,
    start: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MdListItem {
    text: String,
    task: bool,
    checked: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MdTableCell {
    content: String,
    flags: TableCellFlags,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCellFlags {
    header: bool,
    ///  'center' | 'left' | 'right' | null
    align: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MdTableRow {
    content: String,
}
