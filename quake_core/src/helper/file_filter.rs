use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

fn is_with_prefix(entry: &DirEntry, prefix: &String) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(prefix))
        .unwrap_or(false)
}

pub fn filter_by_prefix(path: PathBuf, prefix: String) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if is_with_prefix(&entry, &prefix) {
            files.push(entry.into_path());
        }
    }
    files
}

pub fn type_from_md_path(buf: &PathBuf) -> Option<String> {
    let mut ancestors = buf.ancestors();
    ancestors.next()?;
    let typ = ancestors.next()?.file_name()?;
    let str = typ.to_str()?.to_string();
    Some(str)
}

#[cfg(test)]
mod tests {
    use crate::helper::file_filter::type_from_md_path;
    use std::path::PathBuf;

    #[test]
    fn type_from() {
        let buf = PathBuf::from("examples")
            .join("todo")
            .join("0001-time-support.md");

        let typ = type_from_md_path(&buf).unwrap();
        assert_eq!(typ, "todo".to_string());
    }
}
