use serde_derive::{Deserialize, Serialize};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ENTRY_FILE: Regex = Regex::new(r#"(?P<index>\d{4})-(?P<name>.*).md"#).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChangeEvent {
    pub file_name: String,
    pub extension: String,
    pub parent: String,
    is_entry_file: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rule {
    pub name: String,
    pub description: String,
    /// constructing from `.when()`
    pub condition: Vec<String>,
    /// constructing from `.then()`
    pub action: Vec<String>,
}

impl Rule {
    pub fn name(&mut self, name: &str) -> &mut Rule {
        self.name = name.to_string();
        self
    }

    pub fn description(&mut self, desc: &str) -> &mut Rule {
        self.description = desc.to_string();
        self
    }

    /// todo: abstract to condition
    pub fn when(&mut self) -> &mut Rule {
        self
    }

    /// todo: abstract to action
    pub fn then(&mut self) -> &mut Rule {
        self
    }
}

// by rules
// 1. match quake entry: file by suffix for start with index
// 2. match file for engine?
// generate
pub fn event_to_rule(change: &mut FileChangeEvent) {
    if ENTRY_FILE.is_match(&*change.file_name) {
        change.is_entry_file = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::watcher_rule::{event_to_rule, FileChangeEvent};

    #[test]
    fn match_index_file_with_rule() {
        event_to_rule(&mut FileChangeEvent {
            file_name: "0001-demo.md".to_string(),
            extension: "".to_string(),
            parent: "".to_string(),
            is_entry_file: false,
        });
    }
}
