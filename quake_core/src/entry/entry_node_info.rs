use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntryNodeInfo {
    pub index: usize,
}

impl Default for EntryNodeInfo {
    fn default() -> Self {
        EntryNodeInfo { index: 0 }
    }
}

impl EntryNodeInfo {
    pub fn inc(&mut self) {
        self.index = self.index + 1
    }
}

pub fn entry_info_from_path(entry_info_path: &PathBuf) -> EntryNodeInfo {
    if !entry_info_path.exists() {
        let info = EntryNodeInfo::default();
        fs::write(
            entry_info_path,
            serde_yaml::to_string(&info).expect("cannot serial"),
        )
        .expect("cannot write to file");

        return info;
    }

    let text = fs::read_to_string(&entry_info_path).expect("cannot read entry-info.yaml");
    let entry_info = serde_yaml::from_str(&*text).unwrap();
    entry_info
}
