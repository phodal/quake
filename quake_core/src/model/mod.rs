use std::time::SystemTime;

pub mod book_review;
pub mod meta_object;
pub mod meta_action;
pub mod meta_config;

#[allow(dead_code)]
pub struct EntryDate {
    created: SystemTime,
    updated: SystemTime,
    due_date: SystemTime,
    resolution_date: SystemTime,
}

#[allow(dead_code)]
pub struct Author {
    name: String,
    email: String,
}

