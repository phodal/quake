///
pub enum MetaField {
    Title(String),
    Tagged(String),
    Searchable(String),
    Filterable(String),
}

pub enum MetaType {
    Summary,
    Note,
    Normal,
    Review,
}
