use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use quake_core::parser::action_parser::ActionDefine;
use quake_core::quake_config::QuakeConfig;

use crate::action::entry_paths::EntryPaths;
use crate::action::entry_usecases;
use crate::action::entry_usecases::find_entry_path;
use crate::action::entrysets::Entrysets;
use crate::helper::editor_exec;
use crate::tui::table_process;

pub fn entry_action(expr: &ActionDefine, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    let paths = EntryPaths::init(&conf.workspace, &expr.object);

    // todo: export api for search
    match expr.action.as_str() {
        "add" => {
            let target_file = entry_usecases::create_entry(&conf.workspace, &expr.object, &expr.text)?.0;
            editor_exec::edit_file(conf.editor, format!("{:}", target_file.display()))?;
            entry_usecases::sync_in_path(&paths)?
        }
        "edit" => {
            let target_file = find_entry_path(
                paths.base,
                &expr.object,
                expr.index_from_parameter())?;

            editor_exec::edit_file(conf.editor, format!("{:}", target_file.display()))?;
        }
        "sync" => {
            entry_usecases::sync_in_path(&paths)?
        }
        "dump" => {
            dump_by_path(&paths)?
        }
        "list" => {
            let entries = paths.base.join("entries.csv");
            show_entrysets(&entries);
        }
        _ => {}
    }

    Ok(())
}

fn show_entrysets(path: &PathBuf) {
    let mut rdr = csv::Reader::from_reader(File::open(path).expect("cannot open file"));
    let table = table_process::csv_to_terminal_table(&mut rdr);

    // todo: change to terminal ui
    println!("{}", table);
}

pub fn dump_by_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let map = Entrysets::jsonify(&paths.base)?;
    fs::write("dump.json", map)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use quake_core::parser::action_parser::ActionDefine;
    use quake_core::quake_config::QuakeConfig;

    use crate::cli::action;

    #[test]
    #[ignore]
    fn update_todo() {
        let expr = ActionDefine::from("todo.update(1)").unwrap();
        let mut config = QuakeConfig::default();
        config.workspace = "_fixtures".to_string();
        config.editor = "".to_string();

        action(expr, config).expect("cannot process");
    }
}
