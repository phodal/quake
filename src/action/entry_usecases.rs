use std::error::Error;
use std::fs;
use std::path::PathBuf;

use quake_core::action_parser::ActionDefine;
use quake_core::entry::{EntryDefine, EntryInfo, FrontMatter};
use quake_core::entry::entry_file::EntryFile;

use crate::action::entry_factory;
use crate::action::entry_paths::EntryPaths;
use crate::action::entrysets::Entrysets;

pub fn find_entry_define(expr: &ActionDefine, paths: &EntryPaths) -> EntryDefine {
    let entries: Vec<EntryDefine> = entry_factory::entries_define_from_path(&paths.entries_define).into_iter()
        .filter(|define| {
            define.entry_type.eq(&expr.object)
        })
        .collect();

    let entries_define = if entries.len() == 0 {
        EntryDefine::default()
    } else {
        entries[0].clone()
    };
    entries_define
}

pub fn sync_in_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let (size, content) = Entrysets::generate(&paths.base)?;
    fs::write(&paths.entries, content)?;

    update_entry_info(&paths.entries_info, &mut EntryInfo {
        index: size
    });

    Ok(())
}

pub fn create_entry_file(expr: &ActionDefine, entry_define: &EntryDefine, target_file: &mut PathBuf) {
    let mut entry_file = EntryFile::default();
    let init_map = entry_define.create_title_and_date(expr.text.to_string());
    entry_file.front_matter = FrontMatter { fields: entry_define.merge(init_map) };

    fs::write(&target_file, entry_file.to_string()).expect("cannot write to file");
}

pub fn update_entry_info(entry_info_path: &PathBuf, entry_info: &mut EntryInfo) {
    let result = serde_yaml::to_string(&entry_info).expect("cannot convert to yaml");
    fs::write(&entry_info_path, result).expect("cannot write to file");
}
