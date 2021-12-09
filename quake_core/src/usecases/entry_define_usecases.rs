use crate::entry::{entry_defines, EntryDefine};
use std::path::PathBuf;

pub fn find_entry_define(target_entry: &String, path: &PathBuf) -> EntryDefine {
    let entries: Vec<EntryDefine> = entry_defines::entries_define_from_path(path)
        .into_iter()
        .filter(|define| define.entry_type.eq(target_entry))
        .collect();

    let entries_define = if entries.len() == 0 {
        EntryDefine::default()
    } else {
        entries[0].clone()
    };
    entries_define
}
