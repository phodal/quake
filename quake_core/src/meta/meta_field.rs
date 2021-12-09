use crate::meta::Author;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MetaField {
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

pub enum FormField {
    Checkable,
    Inputtable,
    Editable,
    Selectable,
}

impl Display for MetaField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaField::Text(text) => write!(f, "{}", text),
            MetaField::Title(title) => write!(f, "{}", title),
            MetaField::Author(author) => write!(f, "{:?}", author),
            MetaField::Searchable(str) => write!(f, "{}", str),
            MetaField::Filterable(conds) => write!(f, "{:?}", conds),
            MetaField::Unknown(str) => write!(f, "{}", str),
            MetaField::Date(date) => write!(f, "{}", date),
            MetaField::Body(body) => write!(f, "{}", body),
            MetaField::Theme(theme) => write!(f, "{}", theme),
            MetaField::Epic(epic) => write!(f, "{}", epic),
            MetaField::Priority(priority) => write!(f, "{}", priority),
            MetaField::Attachment(attachment) => write!(f, "{}", attachment),
            MetaField::Flow(flow) => write!(f, "{}", flow),
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
    use crate::meta::meta_field::MetaField;

    #[test]
    fn display_title() {
        let field = MetaField::Title(String::from("Title"));
        assert_eq!("Title", format!("{}", field));
    }
}
