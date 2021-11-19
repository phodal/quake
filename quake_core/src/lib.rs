use serde::{Serialize, Deserialize};

use std::time::SystemTime;

pub mod search_text_parser;

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
    Review
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryBlog {

}

pub struct EntryDate {
    created: SystemTime,
    updated: SystemTime,
    due_date: SystemTime,
    resolution_date: SystemTime,
}

