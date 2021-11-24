use indexmap::IndexMap;
use serde::Deserialize;
use serde_yaml::Value;

pub struct FrontMatter {}

impl FrontMatter {
    /// from markdown file, to parse front matter
    pub fn entry_from_markdown(text: String) -> Option<IndexMap<String, String>> {
        if !text.starts_with("---") {
            return None;
        }

        let split_data = text.split("---").map(Into::into).collect::<Vec<String>>();
        let front_matter = split_data.get(1).expect("parse issue");

        let mut map: IndexMap<String, String> = IndexMap::new();
        for document in serde_yaml::Deserializer::from_str(front_matter) {
            let value = Value::deserialize(document).expect("cannot deserialize");
            if let Value::Mapping(mapping) = value {
                for (v_key, v_value) in mapping {
                    let key = FrontMatter::from_value(v_key);
                    let value = FrontMatter::from_value(v_value);
                    map.insert(key, value);
                }
            }
        }

        Some(map)
    }

    pub fn from_value(value: Value) -> String {
        match value {
            Value::Null => { "".to_string() }
            Value::Bool(bool) => { bool.to_string() }
            Value::Number(num) => { num.to_string() }
            Value::String(string) => { string }
            Value::Sequence(seq) => {
                let seq = seq.into_iter()
                    .map(|value| { FrontMatter::from_value(value) })
                    .collect::<Vec<String>>();

                seq.join(",")
            }
            Value::Mapping(_) => {
                "todo: mapping".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entry_process::front_matter::FrontMatter;

    #[test]
    fn entry_parse() {
        let text = "---
title: hello, world
authors: Phodal HUANG<h@phodal.com>
description: a hello, world
created_date: 2021.11.23
updated_date: 2021.11.21
---

sample

";

        let map = FrontMatter::entry_from_markdown(String::from(text)).expect("parse error");
        assert_eq!("hello, world", map.get("title").unwrap());
        assert_eq!("Phodal HUANG<h@phodal.com>", map.get("authors").unwrap());
        assert_eq!("a hello, world", map.get("description").unwrap());
        assert_eq!("2021.11.23", map.get("created_date").unwrap());
        assert_eq!("2021.11.21", map.get("updated_date").unwrap());
    }
}