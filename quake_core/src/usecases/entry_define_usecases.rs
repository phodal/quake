use crate::entry::{entry_defines, EntryDefine};
use std::path::PathBuf;

pub fn find_entry_define(target_entry: &str, path: &PathBuf) -> EntryDefine {
    let entries: Vec<EntryDefine> = entry_defines::entries_define_from_path(path)
        .into_iter()
        .filter(|define| define.entry_type.eq(target_entry))
        .collect();

    if entries.is_empty() {
        EntryDefine::default()
    } else {
        entries[0].clone()
    }
}
