use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

use tracing::info;

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
use quake_core::errors::QuakeError;
use quake_core::parser::quake::QuakeActionNode;
use quake_core::quake_config::QuakeConfig;
use quake_core::usecases::entry_usecases;
use quake_core::usecases::entry_usecases::find_entry_path;
use quake_core::usecases::entrysets::Entrysets;

use crate::cli::helper::table_process;
use crate::helper::exec_wrapper::{editor_exec, meili_exec};
use crate::usecases::processor_usecases;

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
    Process,
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
            "process" => EntryAction::Process,
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
        "process" => {
            process_content(&paths, &expr.entry, false)?;
        }
        "process::force" => {
            process_content(&paths, &expr.entry, true)?;
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
    let defines = EntryDefines::from_path(&paths.entries_define);
    let define = defines
        .find(entry_type)
        .unwrap_or_else(|| panic!("lost entry define for: {:?}", &entry_type));

    let map = Entrysets::entry_files_by_path(&paths.entry_path, &define)?;

    meili_exec::feed_documents(&conf.search_url, entry_type, &map)?;
    meili_exec::feed_settings(&conf.search_url, &define)?;

    Ok(())
}

fn show_entry_detail(expr: &QuakeActionNode, paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let index = expr.index_from_parameter();
    let target_file = find_entry_path(paths.entry_path.clone(), &expr.entry, index)?;
    info!("show file: {:}", &target_file.display());
    let content = fs::read_to_string(target_file)?;
    let file = EntryFile::from(content.as_str(), index)?;

    highlight_content(format!("{:?}", file.properties).as_str(), "json");

    info!("{:}", file.content);

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
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
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

fn dump_by_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let map = Entrysets::jsonify(&paths.entry_path)?;
    fs::write("dump.json", map)?;
    Ok(())
}

fn process_content(
    paths: &EntryPaths,
    entry_type: &str,
    is_force: bool,
) -> Result<(), Box<dyn Error>> {
    let defines = EntryDefines::from_path(&paths.entries_define);
    if let Some(define) = defines.find(entry_type) {
        processor_usecases::process_by_path(&define, is_force, &paths.entry_path)?
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
        let expr = QuakeActionNode::from_text("papers.process").unwrap();
        let config = QuakeConfig {
            editor: "".to_string(),
            workspace: "examples".to_string(),
            search_url: "".to_string(),
            server_location: "".to_string(),
            // debug_level: "normal".to_string(),
            auto_feed: false,
            port: 0,
        };

        entry_action(&expr, config).unwrap();
    }
}
