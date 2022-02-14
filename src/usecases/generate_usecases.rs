use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::Searcher;
use tracing::{error, info};
use walkdir::{DirEntry, WalkDir};

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::{entry_node_info, EntryDefine, EntryDefines};
use quake_core::errors::QuakeError;
use quake_core::quake::{QuakeTransflowNode, Route};
use quake_core::usecases::entrysets::Entrysets;
use quake_core::QuakeConfig;
use quake_processor::process_engine::ProcessEngine;

pub fn generate_by_flow(flow: &str, config: &QuakeConfig) -> Result<(), Box<dyn Error>> {
    let workspace = PathBuf::from(&config.workspace);
    let flow = format!("transflow generate {{ {:} }}", flow);
    let node = QuakeTransflowNode::from_text(&flow)?;
    let route = &node.routes[0];

    let define = lookup_define(route, &workspace)?;

    let matcher = regex_from_filter(route)?;
    let source_files = files_from_route(route, &matcher)?;

    let entry_path = workspace.join(&define.entry_type);
    fs::create_dir_all(&entry_path)?;

    let mut id = 1;
    process_entries(define, &source_files, &entry_path, &mut id)?;

    // update entry node info
    let entry_info_path = entry_path.join(EntryPaths::entry_info());
    let mut entry_info = entry_node_info::entry_info_from_path(&entry_info_path);
    entry_info.index = id + 1;
    entry_node_info::save_entry_info(&entry_info_path, &mut entry_info);

    // update entries csv
    let (_, content) = Entrysets::generate_csv(&entry_path)?;
    fs::write(entry_path.join(EntryPaths::entries_csv()), content)?;

    Ok(())
}

fn lookup_define(route: &Route, workspace: &Path) -> Result<EntryDefine, Box<dyn Error>> {
    let buf = workspace.join(EntryPaths::entries_define());
    let defines = EntryDefines::from_path(&buf);
    let define = match defines.find(&route.to) {
        None => {
            let err_msg = QuakeError(format!(" lost define of entry {:?}", &route.to));
            return Err(Box::new(err_msg));
        }
        Some(def) => def,
    };

    Ok(define)
}

fn process_entries(
    define: EntryDefine,
    source_files: &[PathBuf],
    target_path: &Path,
    index: &mut usize,
) -> Result<(), Box<dyn Error>> {
    for file in source_files {
        let ext = file.extension().unwrap().to_str().unwrap();
        let engine = ProcessEngine::engine(ext);
        let content = match engine.content(file) {
            Ok(content) => content,
            Err(error) => {
                error!("{:?}", error);
                continue;
            }
        };

        let target_name = file.file_stem().unwrap().to_str().unwrap();

        let mut entry_file = EntryFile::default();
        entry_file.set_properties(define.create_default_properties(target_name));
        entry_file.id = *index;
        entry_file.add_property("file", format!("{:}", file.display()));

        entry_file.content = content;

        let file_name = EntryFile::file_name(*index, target_name);
        let target_file = &target_path.join(file_name);

        info!("save to file: {:?}", target_file.display());
        fs::write(target_file, entry_file.to_string())?;

        *index += 1;
    }

    Ok(())
}

fn files_from_route(route: &Route, matcher: &RegexMatcher) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut source_files = vec![];
    for source in &route.from {
        let source_dir = PathBuf::from(source);
        if !source_dir.exists() {
            let error = QuakeError(format!("path {:?} don't exists", source_dir));
            return Err(Box::new(error));
        }

        for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
            if is_source_file_by_grep(&entry, matcher) {
                source_files.push(entry.into_path());
            }
        }
    }

    Ok(source_files)
}

fn is_source_file_by_grep(entry: &DirEntry, matcher: &RegexMatcher) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| grep_by_text(matcher, s).unwrap_or(false))
        .unwrap_or(false)
}

fn regex_from_filter(route: &Route) -> Result<RegexMatcher, Box<dyn Error>> {
    let filter = match &route.filter {
        None => ".*",
        Some(filter) => filter,
    };

    let matcher = RegexMatcher::new(filter)?;
    Ok(matcher)
}

pub fn grep_by_text(matcher: &RegexMatcher, text: &str) -> Result<bool, Box<dyn Error>> {
    let from = text.as_bytes();
    let mut searcher = Searcher::new();

    let mut has_match = false;
    searcher.search_reader(
        matcher,
        from,
        UTF8(|_, _| {
            has_match = true;
            Ok(true)
        }),
    )?;

    Ok(has_match)
}

#[cfg(test)]
mod tests {
    use grep_regex::RegexMatcher;

    use quake_core::QuakeConfig;

    use crate::generate_by_flow;
    use crate::usecases::generate_usecases::grep_by_text;

    #[ignore]
    #[test]
    fn return_absolute_when_file_exists() {
        let conf = QuakeConfig {
            workspace: "examples".to_string(),
            editor: "".to_string(),
            search_url: "http://127.0.0.1:7700".to_string(),
            server_location: "web".to_string(),
            auto_feed: false,
            port: 8000,
        };

        let flow_text = "from('examples').to('papers').filter('spike.md')";
        if let Err(err) = generate_by_flow(flow_text, &conf) {
            panic!("{:?}", err);
        }
    }

    #[test]
    fn test_grep_regex() {
        let matcher = RegexMatcher::new(".pdf").unwrap();

        assert!(grep_by_text(&matcher, "pdfd.pdf").unwrap_or(false));
        assert!(!grep_by_text(&matcher, "pdf.md").unwrap());
    }
}
