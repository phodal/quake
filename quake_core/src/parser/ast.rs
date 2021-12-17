#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceUnitPart {
    Action(ActionDecl),
    Transflow(TransflowDecl),
    SimpleLayout(SimpleLayoutDecl),
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TransflowDecl {
    pub(crate) name: String,
    pub(crate) flows: Vec<TransflowEnum>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct SimpleLayoutDecl {
    pub(crate) name: String,
    pub(crate) rows: Vec<LayoutColumnNode>,
}

pub type LayoutColumnNode = Vec<LayoutComponentNode>;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct LayoutComponentNode {
    pub(crate) name: String,
    pub(crate) is_empty: bool,
    pub(crate) flow: Option<String>,
    pub(crate) size: i32,
    pub(crate) is_pure_component: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransflowEnum {
    Midway(Midway),
    Endway(Endway),
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Midway {
    pub from: Vec<Parameter>,
    pub end: String,
    pub filter: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransflowSource {
    EntryTypes(Vec<Parameter>),
    RestUrl(FlowUrl),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlowUrl {
    pub url: String,
    /// GET, POST
    pub method: HttpMethod,
    /// http request params
    pub params: Vec<String>,
    /// http body
    pub body: String,
    // to be defined.
    // pub next_rule: NextRule
}

impl Default for FlowUrl {
    fn default() -> Self {
        // todo: add pre check for valid url
        Self {
            url: "".to_string(),
            method: HttpMethod::Get,
            params: vec![],
            body: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Endway {
    pub from: Vec<Parameter>,
    pub component: String,
    pub filter: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ActionDecl {
    pub(crate) action: String,
    pub(crate) object: String,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) text: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Parameter {
    pub value: String,
}
