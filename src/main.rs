extern crate config;

use std::fs;
use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use custom_entry::CustomEntry;

use quake_core::concept_parser::ConceptExpr;
use quake_core::quake_config::QuakeConfig;
use crate::custom_entry::CustomEntries;
use crate::entry_info::EntryInfo;
use crate::slug_helper::slugify;

pub mod cmd;
pub mod custom_entry;
pub mod entry_info;
pub mod slug_helper;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,

    #[clap(short, long)]
    input: String,

    /// custom editor
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
            _ => {}
        }
    }
}

fn create_action(expr: ConceptExpr, conf: QuakeConfig) {
    let config_path = PathBuf::from(conf.path);
    let entries_conf_path = config_path.join("entries.yaml");
    let entries_str = fs::read_to_string(entries_conf_path).expect("cannot read entries.yaml");

    let obj_dir = config_path.join(&expr.object);

    let entry_info_path = obj_dir.join("entry-info.yaml");
    let mut entry_info = process_entry_info(&entry_info_path);

    let _ = fs::create_dir(&obj_dir);

    if expr.object.eq("todo") {
        let entry_file_path = obj_dir.join(format!("{:0>4}-{:}.md", entry_info.index + 1, slugify(&expr.text)));

        let entry = &entries_from_yaml(entries_str).entries[0];
        if expr.action == "add" {
            File::create(&entry_file_path).expect("Unable to create file");
            fs::write(&entry_file_path, entry.front_matter(expr.text)).expect("cannot write to file");

            entry_info.inc();
            let result = serde_yaml::to_string(&entry_info).expect("cannot convert to yaml");
            fs::write(&entry_info_path, result).expect("cannot write to file");
        }

        let file_path = format!("{:}", entry_file_path.display());
        cmd::edit_file(conf.editor, file_path);
    }
}

fn process_entry_info(entry_info_path: &PathBuf) -> EntryInfo {
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

fn entries_from_yaml(text: String) -> CustomEntries {
    let entries: CustomEntries = serde_yaml::from_str(&*text).unwrap();
    entries
}

#[cfg(test)]
mod tests {
    use quake_core::model::meta_object::MetaField;
    use crate::CustomEntry;

    fn custom_entry_from_yaml() -> Vec<CustomEntry> {
        let yaml = "
- type: todo
  display: Todo
  custom_template: quake/todo.yaml
  fields:
    - name: Title
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
        let option = custom_type.field("name").unwrap();
        assert_eq!(&MetaField::Title(String::from("Title")), option)
    }

    #[test]
    fn front_matter() {
        let todo = &custom_entry_from_yaml()[0];

        assert_eq!("---
title: Hello
---
", todo.front_matter(String::from("Hello")));
    }
}
