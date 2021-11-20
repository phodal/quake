use std::time::SystemTime;

pub mod book_review;
pub mod meta_object;
pub mod meta_action;

pub struct EntryDate {
    created: SystemTime,
    updated: SystemTime,
    due_date: SystemTime,
    resolution_date: SystemTime,
}

pub struct EntryAuthor {
    created: String,
    updated: String,
}

