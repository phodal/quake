use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};

use crate::entry::PropMap;
use crate::helper::quake_time;
use crate::meta::{EntryDefineProperties, MetaProperty};

/// Define a new entry:
/// - `entry_type`: the entry_type for operation in system, should be in letter or `_` use in `dir`, `storage` such as
/// - `display`: the name for display
/// - `custom_path`: custom entry path
/// - `properties`: in yaml is a key-value list, need to be convert to HashMap
/// - `processors`: define processors for files
/// - `actions`: custom behavior action, can be use as API #TBD
/// - `flows`: for a simple workflow like **kanban**
/// - `states`: such as **filterable** condition
/// - `component`: display from source
///
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct EntryDefine {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub display: String,
    /// custom path for entries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_path: Option<String>,
    pub properties: Vec<PropMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// file processors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<FileProcessor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<FlowProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<EntryState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
}

/// process file for file content
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct FileProcessor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_engines: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_flows: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct FlowProperty {
    pub property: String,
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct EntryState {
    pub property: String,
    pub items: Vec<String>,
}

impl EntryDefine {
    // todo: directly Deserialize to meta field
    pub fn to_field_type(&self) -> IndexMap<String, MetaProperty> {
        let mut properties: PropMap = IndexMap::new();
        for map in &self.properties {
            for (key, value) in map {
                properties.insert(key.to_string(), value.to_string());
            }
        }

        EntryDefineProperties::from(properties)
    }

    /// set default flow value from first values
    pub fn create_flows_and_states(&self) -> PropMap {
        let mut map: PropMap = IndexMap::new();

        if let Some(list) = &self.flows {
            for flow in list {
                map.insert(flow.property.to_string(), flow.items[0].to_string());
            }
        }

        if let Some(list) = &self.states {
            for flow in list {
                map.insert(flow.property.to_string(), flow.items[0].to_string());
            }
        }

        map
    }

    /// add `title`, `created_date`, `updated_date` value to system
    pub fn create_title_and_date(&self, title: &str) -> PropMap {
        let date = quake_time::date_now();

        let mut map = IndexMap::new();
        map.insert("title".to_string(), title.to_string());
        map.insert("created_date".to_string(), date.clone());
        map.insert("updated_date".to_string(), date);

        map
    }

    /// create default properties with title, date, flows, dates
    pub fn create_default_properties(&self, title: &str) -> PropMap {
        let basic_map = self.create_title_and_date(title);
        let mut properties = self.convert_to_properties(basic_map);
        let flows = self.create_flows_and_states();
        properties.extend(flows);

        properties
    }

    /// merge custom yaml map list to Indexmap
    pub fn convert_to_properties(&self, values: PropMap) -> PropMap {
        let mut result: PropMap = IndexMap::new();

        for field_def in &self.properties {
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
    use crate::meta::MetaProperty;

    fn custom_entry_from_yaml() -> Vec<EntryDefine> {
        let yaml = "
- type: todo
  display: Todo
  properties:
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
        assert_eq!(3, todo.properties.len());

        let custom_type = todo.to_field_type();
        let option = custom_type.get("title").unwrap();
        assert_eq!(&MetaProperty::Title(String::from("Title")), option)
    }

    #[test]
    fn update_properties() {
        let todo = &custom_entry_from_yaml()[0];
        let mut map = IndexMap::new();
        map.insert("title".to_string(), "Hello".to_string());
        map.insert("author".to_string(), "Phodal HUANG".to_string());
        map.insert("date".to_string(), "2021-11-24 19:14:10".to_string());
        map.insert("content".to_string(), "sample".to_string());
        map.insert("new_field".to_string(), "sample".to_string());

        let new_map = todo.convert_to_properties(map);
        assert_eq!("sample", new_map.get("new_field").unwrap())
    }

    #[test]
    fn parse_flowy() {
        let yaml = "
- type: story
  display: Story
  properties:
    - title: Title
    - author: String
    - content: Body
    - status: Flow
    - priority: Flow
    - created_date: Date
    - updated_date: Date
  actions: ~
  flows:
    - property: status
      items: ['Todo', 'Doing', 'Done']
  states:
    - property: priority
      items: ['Low', 'Medium', 'High']

";
        let entries: Vec<EntryDefine> = serde_yaml::from_str(yaml).unwrap();
        let define = entries[0].clone();
        let map = define.create_flows_and_states();

        assert_eq!(map.get("status").unwrap().to_string(), "Todo".to_string());
        assert_eq!(map.get("priority").unwrap().to_string(), "Low".to_string());

        let final_map = define.create_default_properties("hello");
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
