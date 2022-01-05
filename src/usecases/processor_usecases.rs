use std::error::Error;
use std::fs;
use std::path::PathBuf;

use tracing::info;
use walkdir::WalkDir;

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefine;
use quake_core::errors::QuakeError;
use quake_core::meta::MetaProperty;
use quake_processor::process_engine::ProcessEngine;

pub fn process_by_path(paths: &EntryPaths, define: &EntryDefine) -> Result<(), Box<dyn Error>> {
    let file_prop = find_last_file_prop_from_properties(define);
    if file_prop.is_empty() {
        return Err(Box::new(QuakeError("cannot find entry".to_string())));
    }

    let walk_paths = WalkDir::new(&paths.entry_path)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok());

    for path in walk_paths {
        let name = path.file_name().to_str().unwrap();
        if !EntryFile::is_match(name) {
            continue;
        }
        let content = fs::read_to_string(path.path())?;
        let mut entry_file = EntryFile::from(&*content, 1)?;

        if let Some(value) = entry_file.property(&file_prop) {
            let file_path = get_file_property_path(paths, &value);
            if file_path.exists() {
                let ext = file_path.extension().unwrap().to_str().unwrap();
                let engine = ProcessEngine::engine(ext);
                let content = engine.content(&file_path)?;
                info!("call {:?} engine from {:?}", ext, file_path);

                entry_file.content = content;
                fs::write(&path.path(), entry_file.to_string()).unwrap();
            } else {
                return Err(Box::new(QuakeError("cannot entry file".to_string())));
            }
        }
    }

    Ok(())
}

fn find_last_file_prop_from_properties(define: &EntryDefine) -> String {
    let mut field = "".to_string();
    for (typ, property) in define.to_field_type() {
        if let MetaProperty::File(_file) = property {
            field = typ
        }
    }
    field
}

fn get_file_property_path(paths: &EntryPaths, value: &str) -> PathBuf {
    let absolute = PathBuf::from(value);
    if absolute.exists() {
        return absolute;
    }

    paths.entry_path.join(value)
}
