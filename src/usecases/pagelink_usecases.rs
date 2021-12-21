use std::fs;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
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

#[allow(dead_code)]
pub fn generate_links(path: &Path) {
    let defines = EntryDefines::from_path(&path.join(EntryPaths::entries_define()));

    for define in &defines.entries {
        let entry_path = path.join(&define.entry_type);

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

            let content = serde_yaml::to_string(&entry_links).unwrap();
            println!("{:}", content);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::usecases::pagelink_usecases::generate_links;

    #[test]
    fn should_generate_links() {
        let path = PathBuf::from("_fixtures").join("demo_quake");
        generate_links(&path);
    }
}
