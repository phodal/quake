use std::fmt::{Display, Formatter};
use crate::model::{Author, EntryDate};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MetaField {
    Text(String),
    Title(String),
    Tagged(Vec<String>),
    Author(Author),
    Searchable(String),
    EntryDate(EntryDate),
    /// custom filter types
    Filterable(Vec<String>),
    Unknown(String),
}

impl Display for MetaField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaField::Text(text) => write!(f, "{}", text),
            MetaField::Title(title) => write!(f, "{}", title),
            MetaField::Tagged(tag) => write!(f, "{}", tag.join("#")),
            MetaField::Author(author) => write!(f, "{:?}", author),
            MetaField::Searchable(str) => write!(f, "{}", str),
            MetaField::EntryDate(date) => write!(f, "{:?}", date),
            MetaField::Filterable(conds) => write!(f, "{:?}", conds),
            MetaField::Unknown(str) => write!(f, "{}", str),
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
    use crate::model::meta_object::MetaField;

    #[test]
    fn display_title() {
        let field = MetaField::Title(String::from("Title"));
        assert_eq!("Title", format!("{}", field));
    }
}
