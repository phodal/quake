use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct WebComponentElement {
    /// element id, such as `<quake-dashboard>`
    pub id: String,
    /// element's input attributes, such
    /// data in `<quake-dashboard data=""></quake-dashboard>`
    pub attributes: Vec<Attribute>,
    /// output events
    pub events: Vec<EventListener>,
}

impl WebComponentElement {
    pub fn from_js(
        element: &str,
        attributes: Vec<String>,
        events: Vec<String>,
    ) -> WebComponentElement {
        let mut wce = Self::default();
        wce.id = element.to_string();

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
    use crate::transflow::web_component_element::WebComponentElement;

    #[test]
    fn test_web_component_element_struct() {
        let wce = WebComponentElement::from_js(
            "quake-dashboard",
            vec!["data".to_string()],
            vec!["onSave".to_string()],
        );

        assert_eq!("quake-dashboard", wce.id);
        assert_eq!(1, wce.events.len());
        assert_eq!(1, wce.attributes.len());
    }
}
