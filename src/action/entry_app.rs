use std::error::Error;
use std::fs;
use quake_core::action_parser::ActionDefine;
use quake_core::entry::{EntryDefine, EntryDefineFile, EntryInfo, FrontMatter};
use std::path::PathBuf;
use quake_core::entry::entry_file::EntryFile;
use crate::action::entry_action::EntryPaths;
use crate::action::entry_sets::Entrysets;

pub fn find_entry_define(expr: &ActionDefine, paths: &EntryPaths) -> EntryDefine {
    let entries: Vec<EntryDefine> = entries_define_from_path(&paths.entries_define).into_iter()
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

pub fn dump_by_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let map = Entrysets::jsonify(&paths.base)?;

    fs::write("dump.json", map)?;

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

pub fn entries_define_from_path(config_path: &PathBuf) -> Vec<EntryDefine> {
    let entries_str = fs::read_to_string(config_path).expect("cannot read entries-define.yaml");
    let entries: EntryDefineFile = serde_yaml::from_str(&*entries_str).unwrap();

    entries.entries
}

pub fn entry_info_from_path(entry_info_path: &PathBuf) -> EntryInfo {
    if !entry_info_path.exists() {
        let info = EntryInfo::default();
        fs::write(entry_info_path, serde_yaml::to_string(&info).expect("cannot serial")).expect("cannot write to file");

        return info;
    }

    let text = fs::read_to_string(&entry_info_path).expect("cannot read entry-info.yaml");
    let entry_info = serde_yaml::from_str(&*text).unwrap();
    entry_info
}
