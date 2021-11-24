extern crate config;

use std::fs;
use std::fs::File;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use clap::Parser;
use indexmap::IndexMap;

use entry_process::custom_entry::CustomEntries;
use entry_process::custom_entry::CustomEntry;
use entry_process::entry_info::EntryInfo;
use quake_core::input_parser::InputParser;
use quake_core::quake_config::QuakeConfig;

use crate::slug_helper::slugify;

pub mod cmd;
pub mod slug_helper;
pub mod entry_process;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,
    #[clap(short, long)]
    input: String,
    #[clap(short, long, default_value = "")]
    editor: String,
}

fn config(file: &String) -> QuakeConfig {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(file)).unwrap();

    settings.try_into().unwrap()
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut conf: QuakeConfig = config(&opts.config);
    if !opts.editor.is_empty() {
        conf.editor = opts.editor;
    }

    if opts.input.len() > 0 {
        let expr = InputParser::from(opts.input.as_str());
        match expr.object.to_lowercase().as_str() {
            "todo" => {
                create_action(expr, conf);
            }
            _ => {
                create_action(expr, conf);
            }
        }
    }
}

fn create_action(expr: InputParser, conf: QuakeConfig) {
    let config_path = PathBuf::from(conf.path);
    let entry = &entries_from_file(&config_path)[0];

    let obj_dir = config_path.join(&expr.object);
    let _ = fs::create_dir(&obj_dir);

    let entry_info_path = obj_dir.join("entry-info.yaml");
    let mut entry_info = load_entry_info(&entry_info_path);

    let mut entry_file = PathBuf::new();

    match expr.action.as_str() {
        "add" => {
            let string = file_name(entry_info.index + 1, slugify(&expr.text));
            entry_file = obj_dir.join(string);

            File::create(&entry_file).expect("Unable to create file");

            let local: DateTime<Local> = Local::now();
            let date = local.format("%Y-%m-%d %H:%M:%S").to_string();

            let mut map = IndexMap::new();
            map.insert("title".to_string(), expr.text.to_string());
            map.insert("created_date".to_string(), date.clone());
            map.insert("updated_date".to_string(), date);

            fs::write(&entry_file, entry.front_matter(map)).expect("cannot write to file");

            save_entry_info(&entry_info_path, &mut entry_info);
        }
        "update" => {
            // let mut map = IndexMap::new();
            // map.insert("updated_date".to_string(), date);
            // FrontMatter::update_fields(text, map);
        }
        _ => {
            // do_something()
        }
    }

    if entry_file.is_file() {
        let file_path = format!("{:}", entry_file.display());
        cmd::edit_file(conf.editor, file_path);
    } else {
        println!("entry file is noa file");
    }
}

fn file_name(index: usize, text: String) -> String {
    format!("{:0>4}-{:}.md", index, text)
}

fn save_entry_info(entry_info_path: &PathBuf, entry_info: &mut EntryInfo) {
    entry_info.inc();
    let result = serde_yaml::to_string(&entry_info).expect("cannot convert to yaml");
    fs::write(&entry_info_path, result).expect("cannot write to file");
}

fn entries_from_file(config_path: &PathBuf) -> Vec<CustomEntry> {
    let entries_conf_path = config_path.join("entries.yaml");
    let entries_str = fs::read_to_string(entries_conf_path).expect("cannot read entries.yaml");
    let entries: CustomEntries = serde_yaml::from_str(&*entries_str).unwrap();
    let vec = entries.entries;

    vec
}

fn load_entry_info(entry_info_path: &PathBuf) -> EntryInfo {
    if !entry_info_path.exists() {
        let info = EntryInfo::default();
        fs::write(entry_info_path, serde_yaml::to_string(&info).expect("cannot serial")).expect("cannot write to file");

        return info;
    }

    let entry_info_str = fs::read_to_string(&entry_info_path).expect("cannot read entry-info.yaml");
    let entry_info = entry_info_from_yaml(entry_info_str);
    entry_info
}

fn entry_info_from_yaml(text: String) -> EntryInfo {
    serde_yaml::from_str(&*text).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::file_name;

    #[test]
    fn format_test() {
        assert_eq!("0001-hello.md", file_name(1, "hello".to_string()));
        assert_eq!("1111-world.md", file_name(1111, "world".to_string()));
    }
}
