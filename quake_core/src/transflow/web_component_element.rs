#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WebComponentElement {
    pub id: String,
    pub attributes: Vec<Attribute>,
}

impl Default for WebComponentElement {
    fn default() -> Self {
        WebComponentElement {
            id: "".to_string(),
            attributes: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Attribute {
    #[serde(rename = "type")]
    pub typ: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AttributeType {
    Array(Vec<AttributeType>),
    Boolean(bool),
    Number(usize),
    Object,
    Date(String),
    String(String),
}

pub type WcValue = Attribute;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EventListener {
    pub event_name: String,
    pub values: Vec<WcValue>,
}

#[cfg(test)]
mod tests {
    use crate::transflow::web_component_element::WebComponentElement;

    #[test]
    fn mapping_to_file() {
        let _ele = WebComponentElement::default();
    }
}
