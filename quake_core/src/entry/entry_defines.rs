use crate::entry::EntryDefine;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntryDefines {
    pub entries: Vec<EntryDefine>,
}

impl Default for EntryDefines {
    fn default() -> Self {
        EntryDefines {
            entries: vec![]
        }
    }
}

impl EntryDefines {
    pub fn from(text: &str) -> EntryDefines {
        let entries: Vec<EntryDefine> = serde_yaml::from_str(text).unwrap();
        EntryDefines {
            entries
        }
    }
}
