extern crate config;

use std::fs;
use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use custom_entry::CustomEntry;

use quake_core::concept_parser::ConceptExpr;
use quake_core::quake_config::QuakeConfig;
use crate::custom_entry::CustomEntries;

pub mod cmd;
mod custom_entry;

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
    let entries_path = config_path.join("entries.yaml");
    let editor = conf.editor;

    let dir = config_path.join(&expr.object);
    let _ = fs::create_dir(&dir);

    if expr.object.eq("todo") {
        let path = dir.join(format!("{:}.md", 1));

        let string = fs::read_to_string(entries_path).expect("cannot read entries.yaml");
        let entry = &entries_from_yaml(string).entries[0];
        if !&path.exists() {
            File::create(&path).expect("Unable to create file");
            fs::write(&path, entry.front_matter(expr.text)).expect("cannot write to file");
        }

        let file_path = format!("{:}", path.display());
        cmd::edit_file(editor, file_path);
    }
}

pub fn slug(_text: String) {}

fn entries_from_yaml(text: String) -> CustomEntries {
    let entries: CustomEntries = serde_yaml::from_str(&*text).unwrap();
    entries
}

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


#[cfg(test)]
mod tests {
    use quake_core::model::meta_object::MetaField;

    use crate::custom_entry_from_yaml;

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
