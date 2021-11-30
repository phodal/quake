use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use quake_core::parser::action_parser::ActionDefine;
use quake_core::quake_config::QuakeConfig;

use crate::action::entry_usecases;
use crate::action::entry_paths::EntryPaths;
use crate::action::entrysets::Entrysets;
use crate::errors::QuakeError;
use crate::helper::{cmd_runner, file_process};
use crate::tui::table_process;

pub fn entry_action(expr: &ActionDefine, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    let paths = EntryPaths::init(&conf.path, &expr.object);

    // todo: export api for search
    match expr.action.as_str() {
        "add" => {
            let target_file = entry_usecases::create_entry(&conf.path, &expr.object, &expr.text)?.0;
            cmd_runner::edit_file(conf.editor, format!("{:}", target_file.display()))?;
            entry_usecases::sync_in_path(&paths)?
        }
        "edit" => {
            let entry_path = paths.base;

            let index = expr.index_from_parameter();
            let mut target_file = PathBuf::new();

            let prefix = file_process::file_prefix(index);
            let vec = file_process::filter_by_prefix(entry_path, prefix);
            if vec.len() > 0 {
                target_file = vec[0].clone();
            } else {
                return Err(Box::new(QuakeError(format!("cannot find {:} file {:}", expr.object, index))));
            }

            cmd_runner::edit_file(conf.editor, format!("{:}", target_file.display()))?;
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

    let table = table_process::csv_to_table(&mut rdr);
    println!("{}", table);
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
        config.path = "_fixtures".to_string();
        config.editor = "".to_string();

        action(expr, config).expect("cannot process");
    }
}

pub fn dump_by_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let map = Entrysets::jsonify(&paths.base)?;
    fs::write("dump.json", map)?;
    Ok(())
}
