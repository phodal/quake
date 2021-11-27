use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use quake_core::entry::entry_define::{EntryDefine, EntryDefineFile};
use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_info::EntryInfo;
use quake_core::entry::front_matter::FrontMatter;
use quake_core::input_parser::InputParser;
use quake_core::quake_config::QuakeConfig;

use crate::action::{file_process, table_process};
use crate::action::entry_sets::Entrysets;
use crate::helper::cmd;

pub struct EntryPaths {
    pub base: PathBuf,
    pub entries_info: PathBuf,
    pub entries_define: PathBuf,
    pub entries: PathBuf,
}

impl EntryPaths {
    pub fn init(path: &String, object: &String) -> EntryPaths {
        let path = PathBuf::from(path);

        let obj_dir = &path.join(object);
        let _ = fs::create_dir(obj_dir);

        EntryPaths {
            base: PathBuf::from(&obj_dir),
            entries: PathBuf::from(&obj_dir.join("entries.csv")),
            entries_info: PathBuf::from(&obj_dir.join("entries-info.yaml")),
            entries_define: PathBuf::from(&path.join("entries-define.yaml")),
        }
    }
}

pub fn create_action(expr: InputParser, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    let paths = EntryPaths::init(&conf.path, &expr.object);
    let entries_define = &entries_define_from_path(&paths.entries_define)[0];
    let mut entry_info = entry_info_from_path(&paths.entries_info);

    match expr.action.as_str() {
        "add" => {
            let new_md_file = file_process::file_name(entry_info.index + 1, expr.text.as_str());
            let mut target_file = paths.base.join(new_md_file);
            File::create(&target_file)?;

            create_entry_file(&expr, entries_define, &mut target_file);

            entry_info.inc();
            update_entry_info(&paths.entries_info, &mut entry_info);

            cmd::edit_file(conf.editor, format!("{:}", target_file.display()))?;

            sync_in_path(&paths)?
        }
        "edit" => {
            let mut target_file = PathBuf::new();
            let prefix = file_process::file_prefix(expr.index_from_parameter());
            let vec = file_process::filter_by_prefix(paths.base, prefix);
            if vec.len() > 0 {
                target_file = vec[0].clone();
            }

            cmd::edit_file(conf.editor, format!("{:}", target_file.display()))?;
        }
        "sync" => {
            sync_in_path(&paths)?
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

pub fn sync_in_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let (size, content) = Entrysets::generate(&paths.base)?;
    fs::write(&paths.entries, content)?;

    update_entry_info(&paths.entries_info, &mut EntryInfo {
        index: size
    });

    Ok(())
}

pub fn dump_by_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let map = Entrysets::jsonify(&paths.base)?;

    fs::write("dump.json", map)?;

    Ok(())
}

fn create_entry_file(expr: &InputParser, entry_define: &EntryDefine, target_file: &mut PathBuf) {
    let mut entry_file = EntryFile::default();
    let init_map = entry_define.create_title_and_date(expr.text.to_string());
    entry_file.front_matter = FrontMatter { fields: entry_define.merge(init_map) };

    fs::write(&target_file, entry_file.to_string()).expect("cannot write to file");
}

fn show_entrysets(path: &PathBuf) {
    let mut rdr = csv::Reader::from_reader(File::open(path).expect("cannot open file"));

    let table = table_process::csv_to_table(&mut rdr);
    println!("{}", table);
}

fn update_entry_info(entry_info_path: &PathBuf, entry_info: &mut EntryInfo) {
    let result = serde_yaml::to_string(&entry_info).expect("cannot convert to yaml");
    fs::write(&entry_info_path, result).expect("cannot write to file");
}

fn entries_define_from_path(config_path: &PathBuf) -> Vec<EntryDefine> {
    let entries_str = fs::read_to_string(config_path).expect("cannot read entries-define.yaml");
    let entries: EntryDefineFile = serde_yaml::from_str(&*entries_str).unwrap();

    entries.entries
}

fn entry_info_from_path(entry_info_path: &PathBuf) -> EntryInfo {
    if !entry_info_path.exists() {
        let info = EntryInfo::default();
        fs::write(entry_info_path, serde_yaml::to_string(&info).expect("cannot serial")).expect("cannot write to file");

        return info;
    }

    let text = fs::read_to_string(&entry_info_path).expect("cannot read entry-info.yaml");
    let entry_info = serde_yaml::from_str(&*text).unwrap();
    entry_info
}

#[cfg(test)]
mod tests {
    use quake_core::input_parser::InputParser;
    use quake_core::quake_config::QuakeConfig;

    use crate::action::entry_action::create_action;

    #[test]
    #[ignore]
    fn update_todo() {
        let expr = InputParser::from("todo.update(1)");
        let mut config = QuakeConfig::default();
        config.path = "_fixtures".to_string();
        config.editor = "".to_string();

        create_action(expr, config).expect("cannot process");
    }
}
