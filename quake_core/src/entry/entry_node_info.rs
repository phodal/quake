use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntryNodeInfo {
    pub index: usize,
}

impl Default for EntryNodeInfo {
    fn default() -> Self {
        EntryNodeInfo {
            index: 0
        }
    }
}

impl EntryNodeInfo {
    pub fn inc(&mut self) {
        self.index = self.index + 1
    }
}