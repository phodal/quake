use std::fs;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct EntryNodeInfo {
    pub index: usize,
}

impl EntryNodeInfo {
    pub fn new(index: usize) -> EntryNodeInfo {
        EntryNodeInfo { index }
    }
    pub fn inc(&mut self) {
        self.index += 1
    }
}

pub fn entry_info_from_path(entry_info_path: &Path) -> EntryNodeInfo {
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
    serde_yaml::from_str(&*text).unwrap()
}
