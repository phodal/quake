use std::fs;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
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
        let info_str = serde_yaml::to_string(&info).expect("cannot serial");
        fs::write(entry_info_path, info_str).expect("cannot write to file");
        return info;
    }

    let text = fs::read_to_string(&entry_info_path).expect("cannot read entry-info.yaml");
    serde_yaml::from_str(&*text).unwrap()
}

pub fn save_entry_info(entry_info_path: &Path, entry_info: &mut EntryNodeInfo) {
    let result = serde_yaml::to_string(&entry_info).expect("cannot convert to yaml");
    fs::write(&entry_info_path, result).expect("cannot write to file");
}
