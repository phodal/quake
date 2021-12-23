use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::{EntryDefine, EntryDefines};
use quake_core::markdown::entry_reference::PageReference;
use quake_core::markdown::md_processor::MdProcessor;
use quake_core::usecases::entrysets::Entrysets;

/// one entry type's refs
#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct EntryReference {
    pub source_type: String,
    pub source_id: String,
    pub source_title: String,
    pub references: Vec<PageReference>,
}

/// create all entries links
pub fn create_entries_refs(path: &Path) -> Result<(), Box<dyn Error>> {
    let defines = EntryDefines::from_path(&path.join(EntryPaths::entries_define()));

    fs::create_dir_all(
        path.join(EntryPaths::quake())
            .join(EntryPaths::references()),
    )?;

    for define in &defines.entries {
        let entry_path = path.join(&define.entry_type);

        let entry_links = create_entry_reference(define, entry_path)?;

        let content = serde_yaml::to_string(&entry_links)?;
        let path = &path
            .join(EntryPaths::quake())
            .join(EntryPaths::references())
            .join(format!("{:}.yml", &define.entry_type));

        fs::write(path, content)?;
    }

    Ok(())
}

/// create entry type's links
fn create_entry_reference(
    define: &EntryDefine,
    entry_path: PathBuf,
) -> Result<HashMap<String, EntryReference>, Box<dyn Error>> {
    let mut index = 1;
    let mut refs_map: HashMap<String, EntryReference> = HashMap::new();
    for path in &Entrysets::scan_files(&*entry_path) {
        let string = fs::read_to_string(path)?;

        let mut entry_link = EntryReference::default();

        let file = EntryFile::from(&string, index).unwrap();

        if let Ok(links) = MdProcessor::pagelinks(&file.content) {
            entry_link.references = links;
        }

        if !entry_link.references.is_empty() {
            entry_link.source_id = EntryFile::file_prefix(index);
            entry_link.source_title = file.property("title").unwrap();
            entry_link.source_type = define.entry_type.clone();
            // source_id
            refs_map.insert(entry_link.source_id.clone(), entry_link.to_owned());
        }

        index += 1;
    }

    Ok(refs_map)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    use crate::usecases::reference_usecases::{create_entries_refs, EntryReference};

    #[test]
    fn should_generate_links() {
        let path = PathBuf::from("_fixtures").join("demo_quake");
        create_entries_refs(&path).unwrap();
        let buf = path.join("_quake").join("references").join("todo.yml");

        let string = fs::read_to_string(buf).unwrap();
        let refs: HashMap<String, EntryReference> = serde_yaml::from_str(&string).unwrap();

        assert_eq!(1, refs.len());
        let reference = refs.get("0001").unwrap();
        assert_eq!("todo", reference.source_type);
    }
}
