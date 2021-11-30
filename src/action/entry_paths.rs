use std::path::PathBuf;
use std::fs;

pub struct EntryPaths {
    pub base: PathBuf,
    pub entries_info: PathBuf,
    pub entries_define: PathBuf,
    pub entries: PathBuf,
}

impl EntryPaths {
    pub fn init(path: &String, object: &String) -> EntryPaths {
        let path = PathBuf::from(path);

        let obj_dir = &path.join(object);
        let _ = fs::create_dir(obj_dir);

        EntryPaths {
            base: PathBuf::from(&obj_dir),
            entries: PathBuf::from(&obj_dir.join("entries.csv")),
            entries_info: PathBuf::from(&obj_dir.join("entries-info.yaml")),
            entries_define: PathBuf::from(&path.join("entries-define.yaml")),
        }
    }
}
