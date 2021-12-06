use std::error::Error;
use std::path::PathBuf;

use indexmap::IndexMap;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_yaml::Value;

use crate::entry::slug::slugify;
use crate::errors::QuakeError;

#[derive(Deserialize, PartialEq, Debug)]
pub struct EntryFile {
    pub id: usize,
    pub path: PathBuf,
    pub name: String,
    pub fields: IndexMap<String, String>,
    pub content: String,
}

impl Serialize for EntryFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.fields.len()))?;
        for (k, v) in &self.fields {
            map.serialize_entry(&k.to_string(), &v)?;
        }

        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("content", &self.content)?;
        map.end()
    }
}

impl Default for EntryFile {
    fn default() -> Self {
        EntryFile {
            id: 1,
            path: Default::default(),
            name: "".to_string(),
            fields: IndexMap::default(),
            content: "".to_string(),
        }
    }
}

impl ToString for EntryFile {
    fn to_string(&self) -> String {
        let mut output = vec![];
        output.push("---".to_string());
        for (key, value) in &self.fields {
            if !key.eq("content") {
                output.push(format!("{}: {}", key, value));
            }
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

    pub fn id_from_name(file_name: &str) -> Result<usize, Box<dyn Error>> {
        if file_name.len() < 4 {
            return Err(Box::new(QuakeError("length < 4".to_string())));
        }
        let index_str = &file_name[0..4];
        let index: usize = index_str.parse()?;
        Ok(index)
    }

    pub fn from(text: &str, index_id: usize) -> Result<EntryFile, Box<dyn Error>> {
        if !text.starts_with("---") {
            return Ok(EntryFile::default());
        }

        let (front_matter, content) = Self::split_markdown(text);

        let mut fields: IndexMap<String, String> = IndexMap::new();
        for document in serde_yaml::Deserializer::from_str(&front_matter) {
            let value = match Value::deserialize(document) {
                Ok(value) => Ok(value),
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
            id: index_id,
            path: Default::default(),
            name: "".to_string(),
            fields: fields,
            content: String::from(content),
        })
    }

    fn split_markdown(text: &str) -> (String, String) {
        let mut is_in_front_matter = false;
        let mut split_data: Vec<&str> = vec![];
        let mut others: Vec<&str> = vec![];
        for (index, line) in text.lines().enumerate() {
            if (index == 0) & (line == "---") {
                is_in_front_matter = true;
                continue;
            }

            if line == "---" {
                is_in_front_matter = false;
                others.push("");
                continue;
            }

            if is_in_front_matter {
                split_data.push(line);
            } else {
                others.push(line);
            }
        }

        let front_matter = split_data.join("\n");
        others.push("");
        let content = others.join("\n");
        (front_matter, content)
    }

    pub fn header_column(self, index: usize) -> (Vec<String>, Vec<String>) {
        let mut header: Vec<String> = vec![];
        let mut column: Vec<String> = vec![];
        column.push(index.to_string());

        for (key, value) in self.fields {
            if !key.eq("content") {
                header.push(key);
                column.push(value);
            }
        }

        (header, column)
    }

    pub fn insert_id(&mut self, value: usize) {
        self.fields.insert("id".to_string(), value.to_string());
    }

    pub fn field(&self, field: &str) -> Option<String> {
        match self.fields.get(field) {
            None => None,
            Some(err) => return Some(err.to_string()),
        }
    }

    pub fn add_field(&mut self, key: &str, value: &str) {
        self.fields.insert(key.to_string(), value.to_string());
    }

    pub fn set_fields(&mut self, fields: IndexMap<String, String>) {
        self.fields = fields;
    }

    pub fn update_field(&mut self, field: &String, value: &String) {
        match self.fields.get_mut(field) {
            None => {}
            Some(val) => {
                *val = value.to_string();
            }
        };
    }

    pub fn update_content(&mut self, content: &String) {
        if content.starts_with("\n") || content.starts_with("\r\n") {
            self.content = content.to_string();
            return;
        }

        self.content = "\n\n".to_string();
        self.content.push_str(content);
    }
}

pub struct ValueConverter {}

impl ValueConverter {
    pub fn string(value: Value) -> String {
        match value {
            Value::Null => "".to_string(),
            Value::Bool(bool) => bool.to_string(),
            Value::Number(num) => num.to_string(),
            Value::String(string) => string,
            Value::Sequence(seq) => {
                let seq = seq
                    .into_iter()
                    .map(|value| ValueConverter::string(value))
                    .collect::<Vec<String>>();

                seq.join(",")
            }
            Value::Mapping(_) => "todo: mapping".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::entry_file::EntryFile;

    #[test]
    fn parse_id_from_name() {
        let id = EntryFile::id_from_name("0001-demo.md").unwrap();
        assert_eq!(id, 1);

        let msg = EntryFile::id_from_name("000").expect_err("");
        assert_eq!("QuakeError(\"length < 4\")", format!("{:?}", msg));

        let msg = EntryFile::id_from_name("demo.md").expect_err("");
        assert_eq!("ParseIntError { kind: InvalidDigit }", format!("{:?}", msg));
    }

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

        let mut entry_file = EntryFile::from(text, 1).unwrap();

        assert_eq!(text, entry_file.to_string());

        let map = entry_file.fields;
        entry_file.name = "0001-hello-world.md".to_string();

        assert_eq!("hello, world", map.get("title").unwrap());
        assert_eq!("Phodal HUANG<h@phodal.com>", map.get("authors").unwrap());
        assert_eq!("a hello, world", map.get("description").unwrap());
        assert_eq!("2021.11.23", map.get("created_date").unwrap());
        assert_eq!("2021.11.21", map.get("updated_date").unwrap());
    }

    #[test]
    fn to_json() {
        let entry_file = EntryFile::from(demo_text().as_str(), 1).unwrap();
        assert_eq!(
            r#"{"title":"hello, world","authors":"Phodal HUANG<h@phodal.com>","description":"a hello, world","created_date":"2021.11.23","updated_date":"2021.11.21","id":1,"content":"\n\nsample\n\n"}"#,
            serde_json::to_string(&entry_file).unwrap()
        );
    }

    fn demo_text() -> String {
        let text = "---
title: hello, world
authors: Phodal HUANG<h@phodal.com>
description: a hello, world
created_date: 2021.11.23
updated_date: 2021.11.21
---

sample

";
        text.to_string()
    }

    #[test]
    fn update_title() {
        let text = demo_text();
        let mut entry_file = EntryFile::from(text.as_str(), 1).unwrap();

        entry_file.update_field(&"title".to_string(), &"Hello, World".to_string());

        let value = entry_file.fields.get(&"title".to_string()).unwrap();
        assert_eq!(value, &"Hello, World".to_string());
    }

    #[test]
    fn split_front_matter() {
        let text = "---
updated_date: 2021.11.21
---

| sample | fdaf |
|---------|-----|
|   fad  |      |

sample
";
        let result = EntryFile::from(text, 1).unwrap();
        assert_eq!(text, result.to_string());
    }

    #[test]
    fn format_test() {
        let index = 1;
        let text = "hello";
        assert_eq!("0001-hello.md", EntryFile::file_name(index, text));
        let index = 1111;
        let text = "world";
        assert_eq!("1111-world.md", EntryFile::file_name(index, text));
    }
}
