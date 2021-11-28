use chrono::{DateTime, Local};
use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};

use crate::model::CustomType;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntryDefineFile {
    pub entries: Vec<EntryDefine>,
}

impl Default for EntryDefineFile {
    fn default() -> Self {
        EntryDefineFile {
            entries: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EntryDefine {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub display: String,
    pub fields: Vec<IndexMap<String, String>>,
    pub actions: Option<Vec<String>>,
}

impl Default for EntryDefine {
    fn default() -> Self {
        EntryDefine {
            entry_type: "".to_string(),
            display: "".to_string(),
            fields: vec![],
            actions: None
        }
    }
}

impl EntryDefine {
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

    pub fn create_title_and_date(&self, text: String) -> IndexMap<String, String> {
        let local: DateTime<Local> = Local::now();
        let date = local.format("%Y-%m-%d %H:%M:%S").to_string();

        let mut map = IndexMap::new();
        map.insert("title".to_string(), text);
        map.insert("created_date".to_string(), date.clone());
        map.insert("updated_date".to_string(), date);

        map
    }

    pub fn merge(&self, values: IndexMap<String, String>) -> IndexMap<String, String> {
        let mut result: IndexMap<String, String> = IndexMap::new();

        for field_def in &self.fields {
            for (key, _field_type) in field_def {
                let value = if let Some(va) = values.get(key) {
                    va.to_string()
                } else {
                    "".to_string()
                };

                result.insert(key.to_string(), value);
            }
        }

        result.extend(values);

        result
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use crate::entry::entry_define::EntryDefine;
    use crate::model::meta_object::MetaField;

    fn custom_entry_from_yaml() -> Vec<EntryDefine> {
        let yaml = "
- type: todo
  display: Todo
  fields:
    - title: Title
    - content: Body
    - author: Author
";

        let entries: Vec<EntryDefine> = serde_yaml::from_str(yaml).unwrap();
        entries
    }

    #[test]
    fn parse_yaml() {
        let todo = &custom_entry_from_yaml()[0];

        assert_eq!(3, todo.fields.len());

        let custom_type = todo.create_custom_type();
        let option = custom_type.field("title").unwrap();
        assert_eq!(&MetaField::Title(String::from("Title")), option)
    }

    #[test]
    fn update_fields() {
        let todo = &custom_entry_from_yaml()[0];
        let mut map = IndexMap::new();
        map.insert("title".to_string(), "Hello".to_string());
        map.insert("author".to_string(), "Phodal HUANG".to_string());
        map.insert("date".to_string(), "2021-11-24 19:14:10".to_string());
        map.insert("content".to_string(), "sample".to_string());
        map.insert("new_field".to_string(), "sample".to_string());

        let new_map = todo.merge(map);
        assert_eq!("sample", new_map.get("new_field").unwrap())
    }
}
