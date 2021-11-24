use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};

use quake_core::model::CustomType;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CustomEntries {
    pub entries: Vec<CustomEntry>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CustomEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub display: String,
    pub custom_template: String,
    pub fields: Vec<IndexMap<String, String>>,
    pub actions: Option<Vec<String>>,
}

impl CustomEntry {
    pub fn create_custom_type(&self) -> CustomType {
        let mut fields: IndexMap<String, String> = IndexMap::new();
        for map in &self.fields {
            for (key, value) in map {
                fields.insert(key.to_string(), value.to_string());
            }
        }

        let custom_type = CustomType::from(fields);
        custom_type
    }

    pub fn front_matter(&self, values: IndexMap<String, String>) -> String {
        let mut output = vec![];
        for field_def in &self.fields {
            for (key, _field_type) in field_def {
                let value = if let Some(va) = values.get(key) {
                    va.to_string()
                } else {
                    "".to_string()
                };
                output.push(format!("{}:{}", key, value));
            }
        }

        let out = output.join("\n");
        format!("---
{:}
---
", out)
    }
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
