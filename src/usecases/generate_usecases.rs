use std::error::Error;
use std::path::PathBuf;

use quake_core::entry::entry_paths::EntryPaths;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

use quake_core::entry::EntryDefines;
use quake_core::errors::QuakeError;
use quake_core::quake::QuakeTransflowNode;
use quake_core::QuakeConfig;

pub fn generate_by_flow(flow: &str, config: &QuakeConfig) -> Result<(), Box<dyn Error>> {
    let flow = format!("transflow generate {{ {:} }}", flow);
    let node = QuakeTransflowNode::from_text(&flow)?;
    let route = &node.routes[0];

    let buf = PathBuf::from(&config.workspace).join(EntryPaths::entries_define());
    let defines = EntryDefines::from_path(&buf);
    let _define = match defines.find(&route.to) {
        None => {
            let err_msg = QuakeError(format!(" lost define of entry {:?}", &route.to));
            return Err(Box::new(err_msg));
        }
        Some(def) => def,
    };

    let filter = match &route.filter {
        None => ".*",
        Some(filter) => filter,
    };

    let filter_reg = match Regex::new(filter) {
        Ok(reg) => reg,
        Err(err) => {
            let err_msg = QuakeError(format!("compile filter error: {:}", err));
            return Err(Box::new(err_msg));
        }
    };

    let mut source_files = vec![];
    for source in &route.from {
        let source_dir = PathBuf::from(source);
        if !source_dir.exists() {
            let error = QuakeError(format!("path {:?} don't exists", source_dir));
            return Err(Box::new(error));
        }

        fn is_source_file(entry: &DirEntry, filter_reg: &Regex) -> bool {
            entry
                .file_name()
                .to_str()
                .map(|s| filter_reg.is_match(s))
                .unwrap_or(false)
        }

        for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
            if is_source_file(&entry, &filter_reg) {
                source_files.push(entry.into_path());
            }
        }
    }
    println!("{:?}", source_files);

    Ok(())
}

#[cfg(test)]
mod tests {
    use quake_core::QuakeConfig;

    use crate::generate_by_flow;

    #[test]
    fn return_absolute_when_file_exists() {
        let conf = QuakeConfig {
            workspace: "examples".to_string(),
            editor: "".to_string(),
            search_url: "http://127.0.0.1:7700".to_string(),
            server_location: "web".to_string(),
            port: 8000,
        };

        match generate_by_flow("from('examples').to('papers').filter('.*.pdf')", &conf) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
