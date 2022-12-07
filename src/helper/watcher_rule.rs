use quake_core::entry::entry_file::EntryFile;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChangeEvent {
    pub file_name: String,
    pub extension: String,
    // entry type
    pub entry_type: Option<String>,
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
    if EntryFile::is_match(&change.file_name) {
        change.is_entry_file = true;
    }
    // check is generate content when pdf join to path
}

pub enum WatchExtension {
    MD,
    PDF,
    // need a ppt convert to image?
    PPT,
}

#[cfg(test)]
mod tests {
    use crate::helper::watcher_rule::{event_to_rule, FileChangeEvent};

    #[test]
    fn match_index_file_with_rule() {
        event_to_rule(&mut FileChangeEvent {
            file_name: "0001-demo.md".to_string(),
            extension: "".to_string(),
            entry_type: None,
            is_entry_file: false,
        });
    }
}
