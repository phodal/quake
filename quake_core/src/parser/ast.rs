#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceUnitPart {
    Action(ActionDecl),
    Transflow(TransflowDecl),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransflowDecl {
    pub(crate) flows: Vec<Transflow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Transflow {
    Midway(Midway),
    Endway(Endway),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Midway {
    pub from: Vec<String>,
    pub end: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endway {
    pub from: Vec<String>,
    pub component: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionDecl {
    pub(crate) action: String,
    pub(crate) object: String,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) text: String,
}

impl ActionDecl {
    pub fn new() -> ActionDecl {
        ActionDecl {
            action: "".to_string(),
            object: "".to_string(),
            parameters: vec![],
            text: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub value: String,
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            value: "".to_string(),
        }
    }
}
