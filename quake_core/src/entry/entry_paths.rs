use std::fs;
use std::path::PathBuf;

pub struct EntryPaths {
    pub entry_path: PathBuf,
    pub entry_node_info: PathBuf,
    pub entries_define: PathBuf,
    pub entries_csv: PathBuf,
    pub transflows: PathBuf,
}

impl EntryPaths {
    pub fn init(path: &str, object: &str) -> EntryPaths {
        let path = PathBuf::from(path);

        let obj_dir = &path.join(object);
        let _ = fs::create_dir(obj_dir);

        EntryPaths {
            entry_path: PathBuf::from(&obj_dir),
            entries_csv: PathBuf::from(&obj_dir.join("entries.csv")),
            entry_node_info: PathBuf::from(&obj_dir.join("entry-node-info.yaml")),
            entries_define: PathBuf::from(&path.join("entries-define.yaml")),
            transflows: PathBuf::from(&path.join("transflows.yaml")),
        }
    }

    pub fn quake_config() -> &'static str {
        ".quake.yaml"
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

    pub fn references() -> &'static str {
        "references"
    }

    pub fn transfuncs() -> &'static str {
        "transfuncs.js"
    }

    pub fn dashboard_layout() -> &'static str {
        "dashboard.layout"
    }

    pub fn transflow() -> &'static str {
        "transflows.yaml"
    }

    pub fn element_define() -> &'static str {
        "element-define.yml"
    }
}
