use crate::model::{Author, EntryDate};

pub enum MetaField {
    Text(String),
    Title(String),
    Tagged(String),
    Author(Author),
    Searchable(String),
    EntryDate(EntryDate),
    /// custom filter types
    Filterable(Vec<String>),
}

pub enum MetaType {
    Summary,
    Note,
    Normal,
    Review,
}
