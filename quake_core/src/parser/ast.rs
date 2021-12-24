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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Midway {
    pub from: TransflowSource,
    pub end: String,
    pub filter: Option<String>,
    pub map: Option<MapStream>,
}

impl Default for Midway {
    fn default() -> Self {
        Self {
            from: TransflowSource::Empty,
            end: "".to_string(),
            filter: None,
            map: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransflowSource {
    Empty,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endway {
    pub from: TransflowSource,
    pub component: String,
    pub filter: Option<String>,
}

impl Default for Endway {
    fn default() -> Self {
        Self {
            from: TransflowSource::Empty,
            component: "".to_string(),
            filter: None,
        }
    }
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

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct MapStream {
    pub(crate) source: String,
    pub(crate) target: String,
    pub(crate) pipes: Vec<MapPipe>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct MapPipe {
    pub operators: String,
    pub params: Vec<FuncArg>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FuncArg {
    Number(i32),
    String(String),
}
