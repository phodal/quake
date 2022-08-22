use std::fs;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::{EntryDefine, EntryDefines};

pub fn create_suggest(workspace: &str) -> ActionSuggest {
    let mut suggest = ActionSuggest::default();
    let define_path = EntryPaths::entries_define();
    let path = PathBuf::from(workspace).join(define_path);

    let entries_str = fs::read_to_string(path).expect(&*format!("cannot read {:}", define_path));
    let entries: EntryDefines = serde_yaml::from_str(&*entries_str).unwrap();
    suggest.entries = entries.entries;

    let actions = vec!["add", "edit", "show"];
    for action in actions {
        suggest.actions.push(action.to_string());
    }

    suggest
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
pub struct ActionSuggest {
    pub entries: Vec<EntryDefine>,
    pub actions: Vec<String>,
}
