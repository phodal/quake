use crate::meta::Author;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MetaProperty {
    Array(Vec<String>),
    /// title of content, will be default
    Title(String),
    /// body of content, will skill by rules
    Body(String),
    Author(Author),
    Text(String),
    Searchable(String),
    Theme(String),
    Epic(String),
    // String for map
    Date(String),
    /// custom filter types
    Filterable(String),
    /// priority
    Priority(String),
    /// todo: define for Attachment
    Attachment(String),
    /// todo: add flowy
    Flow(String),
    Unknown(String),
}

pub enum FormProperty {
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
            MetaProperty::Filterable(conds) => write!(f, "{:?}", conds),
            MetaProperty::Unknown(str) => write!(f, "{}", str),
            MetaProperty::Date(date) => write!(f, "{}", date),
            MetaProperty::Body(body) => write!(f, "{}", body),
            MetaProperty::Theme(theme) => write!(f, "{}", theme),
            MetaProperty::Epic(epic) => write!(f, "{}", epic),
            MetaProperty::Priority(priority) => write!(f, "{}", priority),
            MetaProperty::Attachment(attachment) => write!(f, "{}", attachment),
            MetaProperty::Flow(flow) => write!(f, "{}", flow),
            MetaProperty::Array(array) => write!(f, "{:?}", array),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
