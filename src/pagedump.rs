use std::fs;
use std::path::PathBuf;

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
use quake_core::usecases::{flow_usecases, layout_usecases};
use quake_core::QuakeConfig;

static DUMP_PATH: &str = "pagedump";

// export data for GitHub pages as demo
pub fn page_dump(conf: QuakeConfig) {
    fs::create_dir_all(DUMP_PATH).unwrap();
    // 1. dump entries config;
    dump_entries_define(&conf);
    // 2. dump quake information;
    dump_transflow(&conf);
    dump_layout(&conf);
    dump_links();
    // 3. export all entry_type data to json
    dump_entries_data();
}

fn dump_transflow(conf: &QuakeConfig) {
    let path = PathBuf::from(&conf.workspace);
    let content = flow_usecases::dump_flows(path);
    let out_path = PathBuf::from(DUMP_PATH).join("transflows.js");

    fs::write(out_path, content).unwrap();
}

fn dump_layout(conf: &QuakeConfig) {
    let path = PathBuf::from(&conf.workspace);
    let out_path = PathBuf::from(DUMP_PATH).join("layout.json");

    if let Ok(layout) = layout_usecases::dump_dashboard_layout(path) {
        let content = serde_json::to_string(&layout).unwrap();
        fs::write(out_path, content).unwrap();
    }
}

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
    use std::path::PathBuf;

    use quake_core::QuakeConfig;

    use crate::page_dump;
    use crate::pagedump::DUMP_PATH;

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

    #[test]
    fn should_dump_transflows() {
        page_dump(config());
        let output = PathBuf::from(DUMP_PATH).join("transflows.js");
        assert!(output.exists());
    }

    #[test]
    fn should_dump_layout() {
        page_dump(config());
        let output = PathBuf::from(DUMP_PATH).join("layout.json");
        assert!(output.exists());
    }
}
