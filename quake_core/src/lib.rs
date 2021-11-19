use std::time::SystemTime;

pub enum EntryType {
    Title(String),
    Tagged(String),
    Searchable(String),
    Filterable(String),
    CreatedDate(String),
    ModifiedDate(String)
}

pub struct EntryDate {
    created: SystemTime,
    modified: SystemTime,
}

