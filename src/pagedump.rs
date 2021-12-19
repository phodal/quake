use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
use quake_core::QuakeConfig;
use std::fs;
use std::path::PathBuf;

static DUMP_PATH: &str = "pagedump";

// export data for GitHub pages as demo
pub fn page_dump(conf: QuakeConfig) {
    fs::create_dir_all(DUMP_PATH).unwrap();
    // 1. dump entries config;
    dump_entries_define(&conf);
    // 2. dump quake information;
    dump_transflow();
    dump_layout();
    dump_links();
    // 3. export all entry_type data to json
    dump_entries_data();
}

fn dump_transflow() {}

fn dump_layout() {}

fn dump_links() {}

fn dump_entries_define(conf: &QuakeConfig) {
    let path = PathBuf::from(&conf.workspace);
    let defines = EntryDefines::from_path(&path.join(EntryPaths::entries_define()));

    let content = serde_json::to_string(&defines).unwrap();
    let out_path = PathBuf::from(DUMP_PATH).join("defines.json");

    fs::write(out_path, content).unwrap();
}

fn dump_entries_data() {}

#[cfg(test)]
mod tests {
    use crate::page_dump;
    use crate::pagedump::DUMP_PATH;
    use quake_core::QuakeConfig;
    use std::path::PathBuf;

    fn config() -> QuakeConfig {
        QuakeConfig {
            workspace: "examples".to_string(),
            editor: "".to_string(),
            search_url: "http://127.0.0.1:7700".to_string(),
            server_location: "web".to_string(),
            port: 8000,
        }
    }

    #[test]
    fn should_dump_entries_define() {
        page_dump(config());
        let output = PathBuf::from(DUMP_PATH).join("defines.json");
        assert!(output.exists());
    }
}
