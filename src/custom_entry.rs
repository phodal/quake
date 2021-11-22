use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use quake_core::model::CustomType;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CustomEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub display: String,
    pub custom_template: String,
    pub fields: Vec<HashMap<String, String>>,
    pub actions: Option<Vec<String>>,
}

impl CustomEntry {
    pub fn create_custom_type(&self) -> CustomType {
        let mut fields: HashMap<String, String> = HashMap::new();
        for map in &self.fields {
            for (key, value) in map {
                fields.insert(key.to_string(), value.to_string());
            }
        }
        let custom_type = CustomType::from(fields);
        custom_type
    }

    pub fn front_matter(&self, title: String) -> String {
        format!("---
title: {:}
---
", title)
    }
}
