use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

use tracing::info;
use walkdir::WalkDir;

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::{EntryDefine, EntryDefines};
use quake_core::errors::QuakeError;
use quake_core::meta::MetaProperty;
use quake_core::parser::quake::QuakeActionNode;
use quake_core::quake_config::QuakeConfig;
use quake_core::usecases::entry_usecases;
use quake_core::usecases::entry_usecases::find_entry_path;
use quake_core::usecases::entrysets::Entrysets;
use quake_processor::process_engine::ProcessEngine;

use crate::cli::helper::table_process;
use crate::helper::exec_wrapper::{editor_exec, meili_exec};

pub enum EntryAction {
    /// add a new entry
    Add,
    /// dump data to json
    Dump,
    /// edit entry by index
    Edit,
    /// feed entry data to search engine
    Feed,
    /// generate auto content
    Generate,
    /// list all entry
    List,
    /// show a entry content
    Show,
    /// sync entry to csv
    Sync,
    /// error content
    Error,
}

impl EntryAction {
    pub fn from(text: &str) -> EntryAction {
        match text {
            "add" => EntryAction::Add,
            "dump" => EntryAction::Dump,
            "edit" => EntryAction::Edit,
            "feed" => EntryAction::Feed,
            "generate" => EntryAction::Generate,
            "list" => EntryAction::List,
            "show" => EntryAction::Show,
            "sync" => EntryAction::Sync,
            _ => EntryAction::Error,
        }
    }
}

pub fn entry_action(expr: &QuakeActionNode, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    let paths = EntryPaths::init(&conf.workspace, &expr.entry);

    match expr.action.as_str() {
        "add" => {
            let target_file =
                entry_usecases::create_entry(&conf.workspace, &expr.entry, &expr.text)?.0;

            if !conf.editor.is_empty() {
                editor_exec::edit_file(conf.editor, format!("{:}", target_file.display()))?;
            }

            entry_usecases::sync_in_path(&paths)?
        }
        "edit" => {
            let file = find_entry_path(paths.entry_path, &expr.entry, expr.index_from_parameter())?;
            if !conf.editor.is_empty() {
                editor_exec::edit_file(conf.editor, format!("{:}", file.display()))?;
            } else {
                return Err(Box::new(QuakeError("editor is empty".to_string())));
            }
        }
        "generate" => {
            generate_content(&paths, &expr.entry)?;
        }
        "sync" => entry_usecases::sync_in_path(&paths)?,
        "feed" => feed_by_path(&paths, &expr.entry, &conf)?,
        "dump" => dump_by_path(&paths)?,
        "show" => show_entry_detail(expr, &paths)?,
        "list" => show_entrysets(&paths.entry_path.join("entries.csv")),
        _ => {
            return Err(Box::new(QuakeError(format!(
                "unknown entry action: {:?}",
                expr
            ))))
        }
    }

    Ok(())
}

fn feed_by_path(
    paths: &EntryPaths,
    entry_type: &str,
    conf: &QuakeConfig,
) -> Result<(), Box<dyn Error>> {
    let temp_file = "dump.json";

    let defines = EntryDefines::from_path(&paths.entries_define);
    let define = defines
        .find(entry_type)
        .unwrap_or_else(|| panic!("lost entry define for: {:?}", &entry_type));

    let map = Entrysets::jsonify_with_format_date(&paths.entry_path, &define)?.to_string();
    fs::write(temp_file, map)?;

    meili_exec::feed_documents(&conf.search_url, entry_type)?;
    meili_exec::feed_settings(&conf.search_url, &define)?;

    fs::remove_file(temp_file)?;

    Ok(())
}

fn show_entry_detail(expr: &QuakeActionNode, paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let index = expr.index_from_parameter();
    let target_file = find_entry_path(paths.entry_path.clone(), &expr.entry, index)?;
    info!("show file: {:}", &target_file.display());
    let content = fs::read_to_string(target_file)?;
    let file = EntryFile::from(content.as_str(), index)?;

    highlight_content(format!("{:?}", file.properties).as_str(), "json");

    println!("{:}", file.content);

    Ok(())
}

fn highlight_content(string: &str, lang: &str) {
    use syntect::easy::HighlightLines;
    use syntect::highlighting::{Style, ThemeSet};
    use syntect::parsing::SyntaxSet;
    use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(lang).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(string) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        println!("{}", escaped);
    }
}

fn show_entrysets(path: &Path) {
    let mut rdr = csv::Reader::from_reader(File::open(path).expect("cannot open file"));
    let table = table_process::csv_to_terminal_table(&mut rdr);

    // todo: change to terminal ui
    println!("{}", table);
}

pub fn dump_by_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let map = Entrysets::jsonify(&paths.entry_path)?;
    fs::write("dump.json", map)?;
    Ok(())
}

fn generate_content(paths: &EntryPaths, entry: &str) -> Result<(), Box<dyn Error>> {
    let defines = EntryDefines::from_path(&paths.entries_define);
    match defines.find(entry) {
        None => {}
        Some(define) => generate_by_path(paths, &define)?,
    }

    Ok(())
}

fn generate_by_path(paths: &EntryPaths, define: &EntryDefine) -> Result<(), Box<dyn Error>> {
    let mut field = "".to_string();
    for (typ, property) in define.to_field_type() {
        if let MetaProperty::File(_file) = property {
            field = typ
        }
    }

    if field.is_empty() {
        return Err(Box::new(QuakeError("cannot find entry".to_string())));
    }

    let walk_paths = WalkDir::new(&paths.entry_path)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok());

    for path in walk_paths {
        let name = path.file_name().to_str().unwrap();
        if EntryFile::is_match(name) {
            let content = fs::read_to_string(path.path())?;
            let mut entry_file = EntryFile::from(&*content, 1)?;
            if let Some(value) = entry_file.property(&field) {
                let file_path = paths.entry_path.join(value);
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
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use quake_core::quake::QuakeActionNode;
    use quake_core::QuakeConfig;

    use crate::cli::entry_action::entry_action;

    #[ignore]
    #[test]
    fn test_generate_content_for_processors() {
        let expr = QuakeActionNode::from_text("papers.generate").unwrap();
        let config = QuakeConfig {
            editor: "".to_string(),
            workspace: "examples".to_string(),
            search_url: "".to_string(),
            server_location: "".to_string(),
            port: 0,
        };

        entry_action(&expr, config).unwrap();
    }
}
