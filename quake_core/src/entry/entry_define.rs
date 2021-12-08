use std::fs;
use std::path::PathBuf;

use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};

use crate::entry::EntryDefines;
use crate::helper::quake_time;
use crate::meta::EntryDefineFields;

/// Define a new entry:
/// - `entry_type`: the entry_type for operation in system, should be in letter or `_` use in `dir`, `storage` such as
/// - `display`: the name for display
/// - `fields`: in yaml is a key-value list, need to be convert to HashMap
/// - `actions`: custom behavior action, can be use as API #TBD
/// - `flows`: use for a simple workflow like **kanban**
/// - `states`: use for a **filterable** condition
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EntryDefine {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub display: String,
    pub fields: Vec<IndexMap<String, String>>,
    pub actions: Option<Vec<String>>,
    pub flows: Option<Vec<EntryFlow>>,
    pub states: Option<Vec<EntryState>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EntryFlow {
    pub field: String,
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EntryState {
    pub field: String,
    pub items: Vec<String>,
}

impl Default for EntryDefine {
    fn default() -> Self {
        EntryDefine {
            entry_type: "".to_string(),
            display: "".to_string(),
            fields: vec![],
            actions: None,
            flows: None,
            states: None,
        }
    }
}

impl EntryDefine {
    pub fn to_type(&self) -> EntryDefineFields {
        let mut fields: IndexMap<String, String> = IndexMap::new();
        for map in &self.fields {
            for (key, value) in map {
                fields.insert(key.to_string(), value.to_string());
            }
        }

        EntryDefineFields::from(fields)
    }

    /// set default flow value from first values
    pub fn create_flows_and_states(&self) -> IndexMap<String, String> {
        let mut map: IndexMap<String, String> = IndexMap::new();

        if let Some(list) = &self.flows {
            for flow in list {
                map.insert(flow.field.to_string(), flow.items[0].to_string());
            }
        }

        if let Some(list) = &self.states {
            for flow in list {
                map.insert(flow.field.to_string(), flow.items[0].to_string());
            }
        }

        map
    }

    /// add `title`, `created_date`, `updated_date` value to system
    pub fn create_title_and_date(&self, title: String) -> IndexMap<String, String> {
        let date = quake_time::date_now();

        let mut map = IndexMap::new();
        map.insert("title".to_string(), title);
        map.insert("created_date".to_string(), date.clone());
        map.insert("updated_date".to_string(), date);

        map
    }

    /// create default fields with title, date, flows, dates
    pub fn create_default_fields(&self, title: String) -> IndexMap<String, String> {
        let basic_map = self.create_title_and_date(title);
        let mut fields = self.merge_to_fields(basic_map);
        let flows = self.create_flows_and_states();
        fields.extend(flows);

        fields
    }

    /// merge custom yaml map list to Indexmap
    pub fn merge_to_fields(&self, values: IndexMap<String, String>) -> IndexMap<String, String> {
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

pub fn entries_define_from_path(config_path: &PathBuf) -> Vec<EntryDefine> {
    let entries_str = fs::read_to_string(config_path).expect("cannot read entries-define.yaml");
    let entries: EntryDefines = serde_yaml::from_str(&*entries_str).unwrap();

    entries.entries
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use crate::entry::entry_define::EntryDefine;
    use crate::meta::MetaField;

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

        let custom_type = todo.to_type();
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

        let new_map = todo.merge_to_fields(map);
        assert_eq!("sample", new_map.get("new_field").unwrap())
    }

    #[test]
    fn parse_flowy() {
        let yaml = "
- type: story
  display: Story
  fields:
    - title: Title
    - author: String
    - content: Body
    - status: Flow
    - priority: Flow
    - created_date: Date
    - updated_date: Date
  actions: ~
  flows:
    - field: status
      items: ['Todo', 'Doing', 'Done']
  states:
    - field: priority
      items: ['Low', 'Medium', 'High']

";
        let entries: Vec<EntryDefine> = serde_yaml::from_str(yaml).unwrap();
        let define = entries[0].clone();
        let map = define.create_flows_and_states();

        assert_eq!(map.get("status").unwrap().to_string(), "Todo".to_string());
        assert_eq!(map.get("priority").unwrap().to_string(), "Low".to_string());

        let final_map = define.create_default_fields("hello".to_string());
        assert_eq!(
            final_map.get("status").unwrap().to_string(),
            "Todo".to_string()
        );
        assert_eq!(
            final_map.get("priority").unwrap().to_string(),
            "Low".to_string()
        );
    }
}
