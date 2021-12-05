#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceUnitPart {
    Action(ActionDecl),
    Transflow(TransflowDecl),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransflowDecl {
    pub(crate) flows: Vec<TransflowEnum>,
}

impl Default for TransflowDecl {
    fn default() -> Self {
        TransflowDecl { flows: vec![] }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransflowEnum {
    Midway(Midway),
    Endway(Endway),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Midway {
    pub from: Vec<Parameter>,
    pub end: String,
}

impl Default for Midway {
    fn default() -> Self {
        Midway {
            from: vec![],
            end: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endway {
    pub from: Vec<Parameter>,
    pub component: String,
}

impl Default for Endway {
    fn default() -> Self {
        Endway {
            from: vec![],
            component: "".to_string(),
        }
    }
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
