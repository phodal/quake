use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntrySetsInfo {
    pub index: usize,
}

impl Default for EntrySetsInfo {
    fn default() -> Self {
        EntrySetsInfo {
            index: 0
        }
    }
}

impl EntrySetsInfo {
    pub fn inc(&mut self) {
        self.index = self.index + 1
    }
}