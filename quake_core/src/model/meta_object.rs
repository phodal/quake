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
    Unknown(String)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MetaType {
    Summary,
    Note,
    Normal,
    Review,
}
