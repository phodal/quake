use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use quake_core::parser::action_parser::ActionDefine;
use quake_core::quake_config::QuakeConfig;

use crate::action::{entry_app, file_process};
use crate::action::entry_paths::EntryPaths;
use crate::cli::quake_action;
use crate::helper::cmd;
use crate::tui::table_process;

pub fn action(expr: ActionDefine, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    if expr.object == "quake" {
        return quake_action::quake_action(expr.action, &conf);
    }

    entry_action(&expr, conf)
}

fn entry_action(expr: &ActionDefine, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    let paths = EntryPaths::init(&conf.path, &expr.object);
    let entries_define = entry_app::find_entry_define(expr, &paths);
    let mut entry_info = entry_app::entry_info_from_path(&paths.entries_info);

    // todo: export api for search
    match expr.action.as_str() {
        "add" => {
            let new_md_file = file_process::file_name(entry_info.index + 1, expr.text.as_str());
            let mut target_file = paths.base.join(new_md_file);
            File::create(&target_file)?;

            entry_app::create_entry_file(&expr, &entries_define, &mut target_file);

            entry_info.inc();
            entry_app::update_entry_info(&paths.entries_info, &mut entry_info);

            cmd::edit_file(conf.editor, format!("{:}", target_file.display()))?;

            entry_app::sync_in_path(&paths)?
        }
        "edit" => {
            let index = expr.index_from_parameter();
            let mut target_file = PathBuf::new();

            let prefix = file_process::file_prefix(index);
            let vec = file_process::filter_by_prefix(paths.base, prefix);
            if vec.len() > 0 {
                target_file = vec[0].clone();
            }

            cmd::edit_file(conf.editor, format!("{:}", target_file.display()))?;
        }
        "sync" => {
            entry_app::sync_in_path(&paths)?
        }
        "dump" => {
            entry_app::dump_by_path(&paths)?
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

    use crate::cli::entry_action::action;

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
