use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;
use quake_core::entry::entry_entity::EntryEntity;

pub fn file_prefix(index: usize) -> String {
    EntryEntity::file_prefix(index)
}

pub fn file_name(index: usize, text: &str) -> String {
    EntryEntity::file_name(index, text)
}

fn is_with_prefix(entry: &DirEntry, prefix: &String) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with(prefix))
        .unwrap_or(false)
}

pub fn filter_by_prefix(path: PathBuf, prefix: String) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in WalkDir::new(path).into_iter()
        .filter_map(|e| e.ok()) {
        if is_with_prefix(&entry, &prefix) {
            files.push(entry.into_path());
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use crate::helper::file_process::file_name;

    #[test]
    fn format_test() {
        assert_eq!("0001-hello.md", file_name(1, "hello"));
        assert_eq!("1111-world.md", file_name(1111, "world"));
    }
}
