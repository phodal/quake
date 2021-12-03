#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceUnitPart {
    Action(ActionDecl),
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
