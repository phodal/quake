use std::error::Error;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeMap;
use serde_yaml::Value;

use crate::entry::front_matter::FrontMatter;
use crate::slug::slugify;

#[derive(Deserialize, PartialEq, Debug)]
pub struct EntryFile {
    pub name: String,
    pub front_matter: FrontMatter,
    pub content: String,
}

impl Default for EntryFile {
    fn default() -> Self {
        EntryFile {
            name: "".to_string(),
            front_matter: FrontMatter::default(),
            content: "".to_string(),
        }
    }
}

impl ToString for EntryFile {
    fn to_string(&self) -> String {
        let mut output = vec![];
        output.push("---".to_string());
        for (key, value) in &self.front_matter.fields {
            output.push(format!("{}: {}", key, value));
        }
        output.push("---".to_string());

        let mut str = output.join("\n");
        str.push_str(&*self.content);

        str
    }
}

impl EntryFile {
    pub fn file_prefix(index: usize) -> String {
        format!("{:0>4}", index)
    }

    pub fn file_name(index: usize, text: &str) -> String {
        format!("{:0>4}-{:}.md", index, slugify(text))
    }

    pub fn from(text: &str) -> Result<EntryFile, Box<dyn Error>> {
        if !text.starts_with("---") {
            return Ok(EntryFile::default())
        }

        let split_data = text.split("---").map(Into::into).collect::<Vec<String>>();
        let front_matter = split_data.get(1).expect("parse issue");
        let content = split_data.get(2).expect("parse issue");

        let mut fields: IndexMap<String, String> = IndexMap::new();
        for document in serde_yaml::Deserializer::from_str(front_matter) {
            let value = match Value::deserialize(document) {
                Ok(value) => { Ok(value) }
                Err(err) => {
                    println!("{}", front_matter);
                    println!("{:?}", err);
                    Err(err)
                }
            }?;
            if let Value::Mapping(mapping) = value {
                for (v_key, v_value) in mapping {
                    let key = ValueConverter::string(v_key);
                    let value = ValueConverter::string(v_value);
                    fields.insert(key, value);
                }
            }
        }

        Ok(EntryFile {
            name: "".to_string(),
            front_matter: FrontMatter { fields },
            content: String::from(content),
        })
    }

    pub fn header_column(self, index: i32) -> (Vec<String>, Vec<String>) {
        let mut header: Vec<String> = vec![];
        let mut column: Vec<String> = vec![];
        column.push(index.to_string());

        for (key, value) in self.front_matter.fields {
            header.push(key);
            column.push(value);
        }

        (header, column)
    }
}

impl Serialize for EntryFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.front_matter.fields.len()))?;
        for (k, v) in &self.front_matter.fields {
            map.serialize_entry(&k.to_string(), &v)?;
        }

        map.serialize_entry("content", &self.content)?;
        map.end()
    }
}

pub struct ValueConverter {}

impl ValueConverter {
    pub fn string(value: Value) -> String {
        match value {
            Value::Null => { "".to_string() }
            Value::Bool(bool) => { bool.to_string() }
            Value::Number(num) => { num.to_string() }
            Value::String(string) => { string }
            Value::Sequence(seq) => {
                let seq = seq.into_iter()
                    .map(|value| { ValueConverter::string(value) })
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
    use crate::entry::entry_file::EntryFile;

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

        let mut entry_file = EntryFile::from(text).unwrap();

        assert_eq!(text, entry_file.to_string());

        let map = entry_file.front_matter.fields;
        entry_file.name = "0001-hello-world.md".to_string();

        assert_eq!("hello, world", map.get("title").unwrap());
        assert_eq!("Phodal HUANG<h@phodal.com>", map.get("authors").unwrap());
        assert_eq!("a hello, world", map.get("description").unwrap());
        assert_eq!("2021.11.23", map.get("created_date").unwrap());
        assert_eq!("2021.11.21", map.get("updated_date").unwrap());
    }


    #[test]
    fn to_json() {
        let text = "---
title: hello, world
authors: Phodal HUANG<h@phodal.com>
description: a hello, world
created_date: 2021.11.23
updated_date: 2021.11.21
---

sample

";

        let entry_file = EntryFile::from(text).unwrap();

        assert_eq!(r#"{"title":"hello, world","authors":"Phodal HUANG<h@phodal.com>","description":"a hello, world","created_date":"2021.11.23","updated_date":"2021.11.21","content":"\n\nsample\n\n"}"#, serde_json::to_string(&entry_file).unwrap());
    }
}