use crate::entry::EntryDefine;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntryDefines {
    pub entries: Vec<EntryDefine>,
}

impl Default for EntryDefines {
    fn default() -> Self {
        EntryDefines { entries: vec![] }
    }
}

impl EntryDefines {
    pub fn from(text: &str) -> EntryDefines {
        let entries: Vec<EntryDefine> = serde_yaml::from_str(text).unwrap();
        EntryDefines { entries }
    }

    pub fn from_path(path: &PathBuf) -> Vec<EntryDefine> {
        entries_define_from_path(path)
    }
}

pub fn entries_define_from_path(config_path: &PathBuf) -> Vec<EntryDefine> {
    let entries_str = fs::read_to_string(config_path).expect("cannot read entries-define.yaml");
    let entries: EntryDefines = serde_yaml::from_str(&*entries_str).unwrap();

    entries.entries
}
