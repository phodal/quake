#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceUnitPart {
    Action(ActionDecl),
    Transflow(TransflowDecl),
    SimpleLayout(SimpleLayoutDecl),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransflowDecl {
    pub(crate) name: String,
    pub(crate) flows: Vec<TransflowEnum>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleLayoutDecl {
    pub(crate) name: String,
    pub(crate) rows: Vec<LayoutColumn>,
}

impl Default for SimpleLayoutDecl {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            rows: vec![],
        }
    }
}

pub type LayoutColumn = Vec<LayoutComponent>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LayoutComponent {
    pub(crate) name: String,
    pub(crate) is_empty: bool,
    pub(crate) flow: Option<String>,
    pub(crate) size: i32,
}

impl Default for LayoutComponent {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            is_empty: false,
            flow: None,
            size: 0,
        }
    }
}

impl Default for TransflowDecl {
    fn default() -> Self {
        TransflowDecl {
            name: "".to_string(),
            flows: vec![],
        }
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
    pub filter: String,
}

impl Default for Midway {
    fn default() -> Self {
        Midway {
            from: vec![],
            end: "".to_string(),
            filter: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endway {
    pub from: Vec<Parameter>,
    pub component: String,
    pub filter: String,
}

impl Default for Endway {
    fn default() -> Self {
        Endway {
            from: vec![],
            component: "".to_string(),
            filter: "".to_string(),
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
