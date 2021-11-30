use std::error::Error;
use std::fs;
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

use quake_core::entry::EntryDefineFile;
use quake_core::QuakeConfig;

use crate::action::entry_usecases;
use crate::action::entry_paths::EntryPaths;
use crate::action::entrysets::Entrysets;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn quake_action(action: String, conf: &QuakeConfig) -> Result<(), Box<dyn Error>> {
    match action.as_str() {
        "sync" => {
            let path = PathBuf::from(&conf.path);

            let mut define_file = EntryDefineFile::default();
            for entry in WalkDir::new(path).min_depth(1).into_iter()
                .filter_entry(|e| !is_hidden(e)) {
                let entry = entry.unwrap();
                if !entry.path().is_dir() {
                    continue
                }
                let path_name = format!("{:}", entry.path().file_name().unwrap().to_str().unwrap());

                let paths = EntryPaths::init(&conf.path, &path_name);
                entry_usecases::sync_in_path(&paths).unwrap();
                let csv = entry.path().join("entries.csv");
                if csv.exists() {
                    define_file.entries.push(Entrysets::define_from_csv(path_name, csv)?);
                }
            }

            let content = serde_yaml::to_string(&define_file).unwrap();
            fs::write(PathBuf::from(&conf.path).join("entries-define.yaml"), content).unwrap();
        }
        _ => {}
    }

    Ok(())
}
