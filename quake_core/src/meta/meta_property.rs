use std::fmt::{Display, Formatter};

use crate::meta::Author;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MetaProperty {
    Enum(Vec<String>),
    /// title of content, will be default
    Title(String),
    /// body of content, will skill by rules
    Content(String),
    Author(Author),
    Text(String),
    Searchable(String),
    /// custom filter types
    Filterable(String),
    /// Hierarchy
    Theme(String),
    Epic(String),
    /// Date as you know
    Date(String),
    /// Data for things
    Data(String),
    /// priority
    Priority(String),
    /// todo: define for Attachment
    Attachment(String),
    /// File
    File(String),
    /// todo: add flowy
    Flow(String),
    /// raw content
    Raw(String),
    Unknown(String),
}

pub enum FormProperty {
    /// for check box
    Checkable,
    Inputtable,
    Editable,
    Selectable,
}

impl Display for MetaProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaProperty::Text(text) => write!(f, "{}", text),
            MetaProperty::Title(title) => write!(f, "{}", title),
            MetaProperty::Author(author) => write!(f, "{:?}", author),
            MetaProperty::Searchable(str) => write!(f, "{}", str),
            MetaProperty::Filterable(cond) => write!(f, "{:?}", cond),
            MetaProperty::Unknown(str) => write!(f, "{}", str),
            MetaProperty::Date(date) => write!(f, "{}", date),
            MetaProperty::Content(body) => write!(f, "{}", body),
            MetaProperty::Theme(theme) => write!(f, "{}", theme),
            MetaProperty::Epic(epic) => write!(f, "{}", epic),
            MetaProperty::Priority(priority) => write!(f, "{}", priority),
            MetaProperty::Attachment(attachment) => write!(f, "{}", attachment),
            MetaProperty::Flow(flow) => write!(f, "{}", flow),
            MetaProperty::Enum(array) => write!(f, "{:?}", array),
            MetaProperty::File(path) => write!(f, "{}", path),
            MetaProperty::Data(data) => write!(f, "{}", data),
            MetaProperty::Raw(data) => write!(f, "{}", data),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MetaType {
    Summary,
    Note,
    Normal,
    Review,
}

#[cfg(test)]
mod tests {
    use crate::meta::meta_property::MetaProperty;

    #[test]
    fn display_title() {
        let field = MetaProperty::Title(String::from("Title"));
        assert_eq!("Title", format!("{}", field));
    }
}
