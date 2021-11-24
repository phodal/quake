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
use quake_core::concept_expr::ConceptExpr;
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
        let expr = ConceptExpr::from(opts.input.as_str());
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

fn create_action(expr: ConceptExpr, conf: QuakeConfig) {
    let config_path = PathBuf::from(conf.path);
    let entry = &entries_from_file(&config_path)[0];

    let obj_dir = config_path.join(&expr.object);
    let _ = fs::create_dir(&obj_dir);

    let entry_info_path = obj_dir.join("entry-info.yaml");
    let mut entry_info = load_entry_info(&entry_info_path);

    let mut entry_file = PathBuf::new();

    match expr.action.as_str() {
        "add" => {
            entry_file = obj_dir.join(format!("{:0>4}-{:}.md", entry_info.index + 1, slugify(&expr.text)));

            File::create(&entry_file).expect("Unable to create file");

            let local: DateTime<Local> = Local::now();
            let created_date = local.format("%Y-%m-%d %H:%M:%S").to_string();

            let mut map = IndexMap::new();
            map.insert("title".to_string(), expr.text.to_string());
            map.insert("created_date".to_string(), created_date);

            fs::write(&entry_file, entry.front_matter(map)).expect("cannot write to file");

            save_entry_info(&entry_info_path, &mut entry_info);
        }
        "update" => {}
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
    let info: EntryInfo = serde_yaml::from_str(&*text).unwrap();
    info
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use quake_core::model::meta_object::MetaField;

    use crate::CustomEntry;

    fn custom_entry_from_yaml() -> Vec<CustomEntry> {
        let yaml = "
- type: todo
  display: Todo
  custom_template: quake/todo.yaml
  fields:
    - title: Title
    - date: EntryDate
    - content: Text
    - author: Author
";

        let entries: Vec<CustomEntry> = serde_yaml::from_str(yaml).unwrap();
        entries
    }

    #[test]
    fn format_test() {
        assert_eq!(format!("{:0>4}", 1), "0001");
        assert_eq!(format!("{:0>4}", 100), "0100");
    }

    #[test]
    fn parse_yaml() {
        let todo = &custom_entry_from_yaml()[0];

        assert_eq!(4, todo.fields.len());

        let custom_type = todo.create_custom_type();
        let option = custom_type.field("title").unwrap();
        assert_eq!(&MetaField::Title(String::from("Title")), option)
    }

    #[test]
    fn front_matter() {
        let todo = &custom_entry_from_yaml()[0];
        let mut map = IndexMap::new();
        map.insert("title".to_string(), "Hello".to_string());

        assert_eq!("---
title:Hello
date:
content:
author:
---
", todo.front_matter(map));
    }
}
