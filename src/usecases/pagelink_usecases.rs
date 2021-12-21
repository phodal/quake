use std::fs;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::{EntryDefine, EntryDefines};
use quake_core::markdown::entry_reference::PageReference;
use quake_core::markdown::md_processor::MdProcessor;
use quake_core::usecases::entrysets::Entrysets;

///one entry type's refs
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct EntryLink {
    pub source_type: String,
    pub source_id: String,
    pub references: Vec<PageReference>,
}

/// create all entries links
#[allow(dead_code)]
pub fn create_entries_links(path: &Path) {
    let defines = EntryDefines::from_path(&path.join(EntryPaths::entries_define()));

    for define in &defines.entries {
        let entry_path = path.join(&define.entry_type);

        let entry_links = create_entry_links(define, entry_path);

        let content = serde_yaml::to_string(&entry_links).unwrap();
        let path = &path
            .join(EntryPaths::quake())
            .join(EntryPaths::links())
            .join(format!("{:}.yml", &define.entry_type));

        fs::write(path, content).unwrap();
    }
}

/// create entry type's links
#[allow(dead_code)]
fn create_entry_links(define: &EntryDefine, entry_path: PathBuf) -> Vec<EntryLink> {
    let mut index = 1;
    let mut entry_links: Vec<EntryLink> = vec![];
    for path in &Entrysets::scan_files(&*entry_path) {
        let string = fs::read_to_string(path).unwrap();

        let mut entry_link = EntryLink::default();
        if let Ok(links) = MdProcessor::pagelinks(&string) {
            entry_link.references = links;
        }

        entry_link.source_id = index.to_string();
        entry_link.source_type = define.entry_type.clone();

        entry_links.push(entry_link);
        index += 1;
    }
    entry_links
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::usecases::pagelink_usecases::{create_entries_links, EntryLink};

    #[test]
    fn should_generate_links() {
        let path = PathBuf::from("_fixtures").join("demo_quake");
        create_entries_links(&path);
        let buf = path.join("_quake").join("links").join("todo.yml");

        let string = fs::read_to_string(buf).unwrap();
        let links: Vec<EntryLink> = serde_yaml::from_str(&string).unwrap();

        assert_eq!(1, links.len());
        assert_eq!("todo", links[0].source_type);
    }
}
