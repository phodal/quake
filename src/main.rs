extern crate config;

use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

use clap::Parser;

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
    println!("{:?}", conf);
    if opts.input.len() > 0 {
        let expr = ConceptExpr::from(opts.input.as_str());
        println!("{:?}", expr);
    }
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

#[cfg(test)]
mod tests {
    use quake_core::concept_parser::ConceptExpr;
    use quake_core::model::meta_object::MetaField;
    use crate::CustomEntry;

    fn custom_entry_from_yaml() -> &CustomEntry {
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
        let todo = &entries[0];
        todo
    }

    #[test]
    fn parse_yaml() {
        let todo = custom_entry_from_yaml();

        assert_eq!(4, todo.fields.len());

        let custom_type = todo.create_custom_type();
        let option = custom_type.field("name").unwrap();
        assert_eq!(&MetaField::Title(String::from("Title")), option)
    }

    #[test]
    fn create_todo() {
        let _expr = ConceptExpr::from("todo.add: hello, world");
    }
}
