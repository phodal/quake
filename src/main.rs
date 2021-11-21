extern crate config;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use serde_derive::{Deserialize, Serialize};

use quake_core::concept_parser::ConceptExpr;
use quake_core::model::CustomType;
use quake_core::quake_config::QuakeConfig;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap(short, long)]
    input: String,
}

fn config(file: &String) -> QuakeConfig {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(file)).unwrap();

    settings.try_into().unwrap()
}

fn main() {
    let opts: Opts = Opts::parse();

    let conf = config(&opts.config);
    if opts.input.len() > 0 {
        let expr = ConceptExpr::from(opts.input.as_str());
        create_todo(expr, conf);
    }
}

fn create_todo(expr: ConceptExpr, conf: QuakeConfig) {
    let config_path = PathBuf::from(conf.path);
    let editor = conf.editor;

    if expr.object.eq("todo") {
        let dir = config_path.join("todo");

        let _ = fs::create_dir(&dir);
        let path = dir
            .join(format!("{:}.md", last_id));

        if !&path.exists() {
            File::create(&path).expect("Unable to create file");
        }

        let file = format!("{:}", path.display());
        edit_file(editor, file);
    }
}

pub fn slug(text: String) {
    
}

fn edit_file(editor: String, file: String) {
    // todo: split os
    Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("{:} {:?}", editor, file))
        // .arg(file)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("failed to execute process");
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CustomEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub display: String,
    pub custom_template: String,
    pub fields: Vec<HashMap<String, String>>,
    pub actions: Option<Vec<String>>,
}

impl CustomEntry {
    pub fn create_custom_type(&self) -> CustomType {
        let mut fields: HashMap<String, String> = HashMap::new();
        for map in &self.fields {
            for (key, value) in map {
                fields.insert(key.to_string(), value.to_string());
            }
        }
        let custom_type = CustomType::from(fields);
        custom_type
    }
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
    use std::fs;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use quake_core::concept_parser::ConceptExpr;
    use quake_core::model::meta_object::MetaField;

    use crate::{custom_entry_from_yaml, CustomEntry};

    #[test]
    fn parse_yaml() {
        let todo = &custom_entry_from_yaml()[0];

        assert_eq!(4, todo.fields.len());

        let custom_type = todo.create_custom_type();
        let option = custom_type.field("name").unwrap();
        assert_eq!(&MetaField::Title(String::from("Title")), option)
    }
}
