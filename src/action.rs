use std::fs;
use std::fs::File;
use std::path::PathBuf;
use comfy_table::Table;

use quake_core::input_parser::InputParser;
use quake_core::quake_config::QuakeConfig;

use crate::entry::entry_define::{EntryDefines, EntryDefine};
use crate::entry::entry_file::EntryFile;
use crate::entry::entry_info::EntryInfo;
use crate::entry::front_matter::FrontMatter;
use crate::helper::cmd;
use crate::helper::slug::slugify;

pub fn create_action(expr: InputParser, conf: QuakeConfig) {
    let config_path = PathBuf::from(conf.path);
    let entry_define = &entry_define_from_path(&config_path)[0];

    let obj_dir = config_path.join(&expr.object);
    let _ = fs::create_dir(&obj_dir);

    let entry_info_path = obj_dir.join("entry-info.yaml");
    let mut entry_info = entry_info_from_path(&entry_info_path);

    let mut entry_path = PathBuf::new();

    match expr.action.as_str() {
        "add" => {
            let string = file_name(entry_info.index + 1, slugify(&expr.text));
            entry_path = obj_dir.join(string);

            File::create(&entry_path).expect("Unable to create file");

            let mut entry_file = EntryFile::default();
            let init_map = entry_define.create_title_and_date(expr.text.to_string());
            entry_file.front_matter = FrontMatter { fields: entry_define.merge_map(init_map) };

            fs::write(&entry_path, entry_file.to_string()).expect("cannot write to file");
            save_entry_info(&entry_info_path, &mut entry_info);
        }
        "update" => {
            // let mut map = IndexMap::new();
            // map.insert("updated_date".to_string(), date);
            // FrontMatter::update_fields(text, map);
        }
        "list" => {
            show_entrysets(&obj_dir);
        }
        _ => {
            // do_something()
        }
    }

    if entry_path.is_file() {
        let file_path = format!("{:}", entry_path.display());
        cmd::edit_file(conf.editor, file_path);
    } else {
        println!("entry file is no a file");
    }
}

pub fn file_name(index: usize, text: String) -> String {
    format!("{:0>4}-{:}.md", index, text)
}

fn show_entrysets(path: &PathBuf)  {
    let entries = path.join("entrysets.csv");
    let file = File::open(entries).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut table = Table::new();

    let mut header = vec![];
    header.push("id".to_string());
    for record in rdr.headers() {
        for str in record {
            header.push(String::from(str))
        }
    }

    table.set_header(header);

    let mut index = 1;
    for result in rdr.records() {
        let record = result.unwrap();
        let mut row = vec![];
        row.push(index.to_string());
        for str in &record {
            row.push(String::from(str));
        }
        index = index + 1;
        table.add_row(row);
    }

    println!("{}", table);
}

fn entry_define_from_path(config_path: &PathBuf) -> Vec<EntryDefine> {
    let entries_conf_path = config_path.join("entries.yaml");
    let entries_str = fs::read_to_string(entries_conf_path).expect("cannot read entries.yaml");
    let entries: EntryDefines = serde_yaml::from_str(&*entries_str).unwrap();
    let vec = entries.entries;

    vec
}

fn save_entry_info(entry_info_path: &PathBuf, entry_info: &mut EntryInfo) {
    entry_info.inc();
    let result = serde_yaml::to_string(&entry_info).expect("cannot convert to yaml");
    fs::write(&entry_info_path, result).expect("cannot write to file");
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
    use crate::action::file_name;

    #[test]
    fn format_test() {
        assert_eq!("0001-hello.md", file_name(1, "hello".to_string()));
        assert_eq!("1111-world.md", file_name(1111, "world".to_string()));
    }
}