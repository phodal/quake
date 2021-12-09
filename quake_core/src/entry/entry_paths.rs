use std::fs;
use std::path::PathBuf;

pub struct EntryPaths {
    pub base: PathBuf,
    pub entry_node_info: PathBuf,
    pub entries_define: PathBuf,
    pub entries_csv: PathBuf,
    pub transflows: PathBuf,
}

impl EntryPaths {
    pub fn init(path: &String, object: &String) -> EntryPaths {
        let path = PathBuf::from(path);

        let obj_dir = &path.join(object);
        let _ = fs::create_dir(obj_dir);

        EntryPaths {
            base: PathBuf::from(&obj_dir),
            entries_csv: PathBuf::from(&obj_dir.join("entries.csv")),
            entry_node_info: PathBuf::from(&obj_dir.join("entry-node-info.yaml")),
            entries_define: PathBuf::from(&path.join("entries-define.yaml")),
            transflows: PathBuf::from(&path.join("transflows.yaml")),
        }
    }

    pub fn entries_define() -> &'static str {
        "entries-define.yaml"
    }

    pub fn entries_csv() -> &'static str {
        "entries.csv"
    }

    pub fn quake() -> &'static str {
        "_quake"
    }

    pub fn transfuncs() -> &'static str {
        "transfuncs.js"
    }
}
