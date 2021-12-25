use indexmap::IndexMap;
use serde::Deserialize;
use std::collections::HashMap;

pub type ElementDefines = Vec<ElementDefine>;

pub fn filter_element_define(defines: &[ElementDefine], key: &str) -> Option<ElementDefine> {
    let def = defines
        .iter()
        .filter(|define| define.name == key)
        .collect::<Vec<&ElementDefine>>();

    if def.is_empty() {
        None
    } else {
        Some(def[0].clone())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct ElementDefine {
    /// element id, such as `<quake-dashboard>`
    pub name: String,
    /// element's input attributes, such
    /// data in `<quake-dashboard data=""></quake-dashboard>`
    pub attributes: Vec<Attribute>,
    /// output events
    pub events: Vec<EventListener>,
    /// data attributes
    pub data_properties: Vec<HashMap<String, String>>,
}

impl ElementDefine {
    pub fn new(id: String) -> Self {
        Self {
            name: id,
            attributes: vec![],
            events: vec![],
            data_properties: vec![],
        }
    }

    pub fn data_map(&self) -> IndexMap<String, String> {
        let mut result: IndexMap<String, String> = IndexMap::new();
        for map in &self.data_properties {
            for (key, value) in map {
                result.insert(key.to_string(), value.to_string());
            }
        }

        result
    }

    pub fn from_js(element: &str, attributes: Vec<String>, events: Vec<String>) -> ElementDefine {
        let mut wce = Self::new(element.to_string());

        for attr in attributes {
            wce.attributes.push(Attribute {
                typ: None,
                name: attr,
            })
        }

        for event in events {
            wce.events.push(EventListener {
                event_name: event,
                event_data: None,
            })
        }

        wce
    }

    pub fn add_event(&mut self, event_name: &str) {
        self.events.push(EventListener {
            event_name: event_name.to_string(),
            event_data: None,
        });
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Attribute {
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AttributeType {
    Array(Vec<AttributeType>),
    Boolean(bool),
    Number(usize),
    String(String),
    Date(String),
    Object(HashMap<String, AttributeType>),
}

pub type EventValue = Attribute;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EventListener {
    pub event_name: String,
    /// to get `event.detail`
    pub event_data: Option<Vec<EventValue>>,
}

#[cfg(test)]
mod tests {
    use crate::entry::entry_paths::EntryPaths;
    use std::fs;
    use std::path::PathBuf;

    use crate::transflow::element_define::ElementDefine;

    #[test]
    fn serialize_wc_element() {
        let quake_path = PathBuf::from("..")
            .join("_fixtures")
            .join("demo_quake")
            .join("_quake")
            .join(EntryPaths::element_define());

        let string = fs::read_to_string(quake_path).unwrap();
        let elements: Vec<ElementDefine> = serde_yaml::from_str(&*string).unwrap();

        assert_eq!("quake-calendar", elements[0].name);

        let map = elements[0].data_map();
        assert_eq!("String", map.get("title").unwrap());
        assert_eq!("String", map.get("content").unwrap());
    }

    #[test]
    fn test_web_component_element_struct() {
        let wce = ElementDefine::from_js(
            "quake-dashboard",
            vec!["data".to_string()],
            vec!["onSave".to_string()],
        );

        assert_eq!("quake-dashboard", wce.name);
        assert_eq!(1, wce.events.len());
        assert_eq!(1, wce.attributes.len());
    }
}
