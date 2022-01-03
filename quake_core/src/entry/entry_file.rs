use std::error::Error;
use std::path::PathBuf;

use indexmap::IndexMap;
use lazy_static::lazy_static;
use regex::Regex;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_yaml::{Sequence, Value};

use crate::entry::slug::slugify;
use crate::errors::QuakeError;
use crate::helper::date_now;
use crate::meta::quake_change::QuakeChange;

lazy_static! {
    static ref ENTRY_FILE: Regex = Regex::new(r#"(?P<index>\d{4})-(?P<name>.*).md"#).unwrap();
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct EntryFile {
    pub id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    pub name: String,
    pub properties: IndexMap<String, String>,
    pub content: String,
    pub changes: Vec<QuakeChange>,
}

impl Serialize for EntryFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.properties.len()))?;
        for (k, v) in &self.properties {
            map.serialize_entry(&k.to_string(), &v)?;
        }

        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("content", &self.content)?;

        if !self.changes.is_empty() {
            let mut vec: Sequence = vec![];
            for change in &self.changes {
                vec.push(Value::from(format!("{:}", change)));
            }

            map.serialize_entry("quake_change", &vec)?;
        }

        map.end()
    }
}

impl Default for EntryFile {
    fn default() -> Self {
        EntryFile {
            id: 1,
            path: None,
            name: "".to_string(),
            properties: IndexMap::default(),
            content: "".to_string(),
            changes: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EntryFileChange {
    pub quake_change: Vec<QuakeChange>,
}

impl ToString for EntryFile {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str("---\n");

        for (key, value) in &self.properties {
            if !key.eq("content") {
                output.push_str(format!("{}: {}\n", key, value).as_str());
            }
        }

        if !self.changes.is_empty() {
            output.push_str("quake_change:\n");
            for change in &self.changes {
                output.push_str(format!("  - {:}\n", change).as_str());
            }
        }

        output.push_str("---");
        output.push_str(&*self.content);

        output
    }
}

impl EntryFile {
    pub fn file_prefix(index: usize) -> String {
        format!("{:0>4}", index)
    }

    pub fn file_name(index: usize, text: &str) -> String {
        format!("{:0>4}-{:}.md", index, slugify(text))
    }

    pub fn is_match(file_name: &str) -> bool {
        ENTRY_FILE.is_match(&*file_name)
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
        let mut changes = vec![];
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
                    if v_key == "quake_change" {
                        changes = ValueConverter::changing(v_value);
                        continue;
                    }

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
            properties: fields,
            content,
            changes,
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
        let mut column: Vec<String> = vec![index.to_string()];

        for (key, value) in self.properties {
            if !key.eq("content") {
                header.push(key);
                column.push(value);
            }
        }

        (header, column)
    }

    pub fn insert_id(&mut self, value: usize) {
        self.properties.insert("id".to_string(), value.to_string());
    }

    pub fn property(&self, field: &str) -> Option<String> {
        self.properties.get(field).map(|err| err.to_string())
    }

    pub fn add_property(&mut self, key: &str, value: String) {
        self.properties.insert(key.to_string(), value);
    }

    pub fn set_properties(&mut self, fields: IndexMap<String, String>) {
        self.properties = fields;
    }

    pub fn update_property(&mut self, field: &str, value: &str) {
        match self.properties.get_mut(field) {
            None => {}
            Some(val) => {
                *val = value.to_string();
            }
        };
    }

    pub fn update_content(&mut self, content: &str) {
        if content.starts_with('\n') || content.starts_with("\r\n") {
            self.content = content.to_string();
            return;
        }

        self.content = "\n\n".to_string();
        self.content.push_str(content);
    }

    /// add status change to quake_change
    pub fn change(&mut self, from: &str, to: &str) {
        self.changes.push(QuakeChange {
            from: from.to_string(),
            to: to.to_string(),
            changed_date: date_now(),
        })
    }
}

pub struct ValueConverter {}

impl ValueConverter {
    pub fn changing(value: Value) -> Vec<QuakeChange> {
        let mut vec = vec![];

        if let Value::Sequence(seq) = value {
            for value in seq {
                let string = ValueConverter::string(value);
                if let Some(change) = QuakeChange::from(string.as_str()) {
                    vec.push(change);
                }
            }
        };

        vec
    }

    pub fn string(value: Value) -> String {
        match value {
            Value::Null => "".to_string(),
            Value::Bool(bool) => bool.to_string(),
            Value::Number(num) => num.to_string(),
            Value::String(string) => string,
            Value::Sequence(seq) => {
                let seq = seq
                    .into_iter()
                    .map(ValueConverter::string)
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

        let map = entry_file.properties;
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

        entry_file.update_property("title", "Hello, World");

        let value = entry_file.properties.get(&"title".to_string()).unwrap();
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

    #[test]
    fn show_change_logging() {
        let text = "---
updated_date: 2021.11.21
quake_change:
  - 2021-12-09 09:32:28 \"\" -> \"Todo\"
  - 2021-12-09 09:40:28 \"Spike\" -> \"Todo\"
  - 2021-12-10 12:12:28 \"Todo\" -> \"Doing\"
  - 2021-12-10 12:12:28 \"Doing\" -> \"Done\"
---

| sample | fdaf |
|---------|-----|
|   fad  |      |

sample
";
        let entry_file = EntryFile::from(text, 1).unwrap();

        assert_eq!(4, entry_file.changes.len());
        assert_eq!(text, entry_file.to_string());
    }

    #[test]
    fn add_change_logging() {
        let text = "---
updated_date: 2021.11.21
quake_change:
  - 2021-12-09 09:32:28 \"\" -> \"Todo\"
  - 2021-12-09 09:40:28 \"Spike\" -> \"Todo\"
  - 2021-12-10 12:12:28 \"Todo\" -> \"Doing\"
---

| sample | fdaf |
|---------|-----|
|   fad  |      |

sample
";
        let mut entry_file = EntryFile::from(text, 1).unwrap();
        assert_eq!(3, entry_file.changes.len());

        assert_eq!(text, entry_file.to_string());
        entry_file.change("Doing", "Done");

        assert_eq!(4, entry_file.changes.len());
    }
}
