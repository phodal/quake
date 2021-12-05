use std::error::Error;
use std::fs;
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

use quake_core::entry::entry_defines::EntryDefines;
use quake_core::QuakeConfig;

use crate::action::entry_usecases;
use crate::action::entrysets::Entrysets;
use crate::helper::meili_exec;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::errors::QuakeError;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn quake_action(action: String, conf: &QuakeConfig) -> Result<(), Box<dyn Error>> {
    match action.as_str() {
        "sync" => {
            sync_defines(&conf)?;
        }
        "migration" => {
            // todo: add migrations for on entries
        }
        "feed" => {
            feed_data(&conf)?;
        }
        _ => {
            return Err(Box::new(QuakeError(format!(
                "unknow quake action: {:?}",
                action
            ))))
        }
    }

    Ok(())
}

fn feed_data(conf: &&QuakeConfig) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(&conf.workspace);
    let temp_file = "dump.json";

    for entry in WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry.unwrap();
        if !entry.path().is_dir() {
            continue;
        }

        let table = entry.path().join("entries.csv");
        if !table.exists() {
            continue;
        }

        let path_name = format!("{:}", entry.path().file_name().unwrap().to_str().unwrap());
        let paths = EntryPaths::init(&conf.workspace, &path_name);

        let map = Entrysets::jsonify(&paths.base)?;
        fs::write(temp_file, map)?;

        meili_exec::feed_command(&path_name)?;
        meili_exec::feed_settings(&path_name)?;

        println!("done '{:}' feed", &path_name);
    }

    fs::remove_file(temp_file)?;

    Ok(())
}

fn sync_defines(conf: &&QuakeConfig) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(&conf.workspace);

    let mut define_file = EntryDefines::default();
    for entry in WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = entry.unwrap();
        if !entry.path().is_dir() {
            continue;
        }

        let path_name = format!("{:}", entry.path().file_name().unwrap().to_str().unwrap());
        if path_name.eq(&conf.server_location) {
            continue;
        }

        let paths = EntryPaths::init(&conf.workspace, &path_name);
        entry_usecases::sync_in_path(&paths)?;

        let csv = entry.path().join("entries.csv");
        if csv.exists() {
            define_file
                .entries
                .push(Entrysets::define_from_csv(path_name, csv)?);
        }
    }

    let content = serde_yaml::to_string(&define_file).unwrap();
    fs::write(
        PathBuf::from(&conf.workspace).join("entries-define.yaml"),
        content,
    )?;

    Ok(())
}
