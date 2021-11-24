use indexmap::IndexMap;
use serde::Deserialize;
use serde_yaml::Value;

pub struct EntryFile {
    pub front_matter: IndexMap<String, String>,
    pub content: String,
}

impl Default for EntryFile {
    fn default() -> Self {
        EntryFile {
            front_matter: IndexMap::new(),
            content: "".to_string()
        }
    }
}

impl ToString for EntryFile {
    fn to_string(&self) -> String {
        let mut output = vec![];
        output.push("---".to_string());
        for (key, value) in &self.front_matter {
            output.push(format!("{}: {}", key, value));
        }
        output.push("---".to_string());

        let mut str = output.join("\n");
        str.push_str(&*self.content);

        str
    }
}

impl EntryFile {
    pub fn from(text: &str) -> EntryFile {
        if !text.starts_with("---") {
            return EntryFile { front_matter: IndexMap::new(), content: String::from(text) };
        }

        let split_data = text.split("---").map(Into::into).collect::<Vec<String>>();
        let front_matter = split_data.get(1).expect("parse issue");
        let content = split_data.get(2).expect("parse issue");

        let mut map: IndexMap<String, String> = IndexMap::new();
        for document in serde_yaml::Deserializer::from_str(front_matter) {
            let value = Value::deserialize(document).expect("cannot deserialize");
            if let Value::Mapping(mapping) = value {
                for (v_key, v_value) in mapping {
                    let key = FrontMatter::string(v_key);
                    let value = FrontMatter::string(v_value);
                    map.insert(key, value);
                }
            }
        }

        EntryFile {
            front_matter: map,
            content: String::from(content),
        }
    }

    pub fn header_column(self, index: i32) -> (Vec<String>, Vec<String>) {
        let mut header: Vec<String> = vec![];
        let mut column: Vec<String> = vec![];
        column.push(index.to_string());

        for (key, value) in self.front_matter {
            header.push(key);
            column.push(value);
        }

        (header, column)
    }
}

pub struct FrontMatter {}

impl FrontMatter {
    pub fn string(value: Value) -> String {
        match value {
            Value::Null => { "".to_string() }
            Value::Bool(bool) => { bool.to_string() }
            Value::Number(num) => { num.to_string() }
            Value::String(string) => { string }
            Value::Sequence(seq) => {
                let seq = seq.into_iter()
                    .map(|value| { FrontMatter::string(value) })
                    .collect::<Vec<String>>();

                seq.join(",")
            }
            Value::Mapping(_) => {
                "todo: mapping".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::entry_file::{EntryFile};

    #[test]
    fn entry_parse() {
        let text = "---
title: hello, world
authors: Phodal HUANG<h@phodal.com>
description: a hello, world
created_date: 2021.11.23
updated_date: 2021.11.21
---

sample

";

        let entry_file = EntryFile::from(text);

        assert_eq!(text, entry_file.to_string());

        let map = entry_file.front_matter;
        assert_eq!("hello, world", map.get("title").unwrap());
        assert_eq!("Phodal HUANG<h@phodal.com>", map.get("authors").unwrap());
        assert_eq!("a hello, world", map.get("description").unwrap());
        assert_eq!("2021.11.23", map.get("created_date").unwrap());
        assert_eq!("2021.11.21", map.get("updated_date").unwrap());
    }
}