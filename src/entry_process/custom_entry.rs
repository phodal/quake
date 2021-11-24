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

    pub fn front_matter(&self, map: IndexMap<String, String>) -> String {
        let mut output = vec![];
        for field_def in &self.fields {
            for (key, _field_type) in field_def {
                let value = if let Some(va) = map.get(key) {
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
