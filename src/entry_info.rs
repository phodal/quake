use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntryInfo {
    pub index: usize,
}

impl Default for EntryInfo {
    fn default() -> Self {
        EntryInfo {
            index: 1
        }
    }
}

impl EntryInfo {
    pub fn inc(&mut self) {
        self.index = self.index + 1
    }
}