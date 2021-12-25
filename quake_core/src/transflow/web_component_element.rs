use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct WebComponentElement {
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

impl WebComponentElement {
    pub fn new(id: String) -> Self {
        Self {
            name: id,
            attributes: vec![],
            events: vec![],
            data_properties: vec![],
        }
    }

    pub fn from_js(
        element: &str,
        attributes: Vec<String>,
        events: Vec<String>,
    ) -> WebComponentElement {
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EventListener {
    pub event_name: String,
    /// to get `event.detail`
    pub event_data: Option<Vec<EventValue>>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::transflow::web_component_element::WebComponentElement;

    #[test]
    fn serialize_wc_element() {
        let quake_path = PathBuf::from("..")
            .join("_fixtures")
            .join("demo_quake")
            .join("elements-define.yml");

        let string = fs::read_to_string(quake_path).unwrap();
        let elements: Vec<WebComponentElement> = serde_yaml::from_str(&*string).unwrap();

        assert_eq!("quake-calendar", elements[0].name);
    }

    #[test]
    fn test_web_component_element_struct() {
        let wce = WebComponentElement::from_js(
            "quake-dashboard",
            vec!["data".to_string()],
            vec!["onSave".to_string()],
        );

        assert_eq!("quake-dashboard", wce.name);
        assert_eq!(1, wce.events.len());
        assert_eq!(1, wce.attributes.len());
    }
}
