use quake_core::entry::entry_file::EntryFile;
use std::fs;
use std::path::PathBuf;

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
use quake_core::usecases::entrysets::Entrysets;
use quake_core::usecases::{flow_usecases, layout_usecases};
use quake_core::QuakeConfig;

static DUMP_PATH: &str = "pagedump";

// export data for GitHub pages as demo
pub fn page_dump(conf: QuakeConfig) {
    // init dir
    if PathBuf::from(DUMP_PATH).exists() {
        fs::remove_dir_all(DUMP_PATH).expect("dir not exists");
    }

    fs::create_dir_all(DUMP_PATH).unwrap();
    fs::create_dir_all(PathBuf::from(DUMP_PATH).join("entry")).unwrap();

    // 1. dump entries config;
    dump_entries_define(&conf);
    // 2. dump quake information;
    dump_transflow(&conf);
    dump_layout(&conf);
    dump_links(&conf);
    // 3. export all entry_type data to json
    dump_entries_data(&conf);
}

fn dump_transflow(conf: &QuakeConfig) {
    let path = PathBuf::from(&conf.workspace);
    let out_path = PathBuf::from(DUMP_PATH).join("transflow").join("script");

    // dump gen code
    if let Ok(content) = flow_usecases::dump_flows(path.clone()) {
        fs::create_dir_all(&out_path).unwrap();

        let out_file = out_path.join("gen_code.js");

        fs::write(out_file, content).unwrap();
    }

    // copy for define load
    let transfuncs = path.join(EntryPaths::quake()).join("transfuncs.js");
    if transfuncs.exists() {
        fs::copy(transfuncs, out_path.join("custom_scripts.js")).unwrap();
    }
}

fn dump_layout(conf: &QuakeConfig) {
    let path = PathBuf::from(&conf.workspace);

    let out_layout_path = PathBuf::from(DUMP_PATH).join("layout");
    fs::create_dir_all(&out_layout_path).unwrap();

    let out_path = out_layout_path.join("dashboard");

    if let Ok(layout) = layout_usecases::dump_dashboard_layout(path) {
        let content = serde_json::to_string(&layout).unwrap();
        fs::write(out_path, content).unwrap();
    }
}

fn dump_links(_conf: &QuakeConfig) {}

fn dump_entries_define(conf: &QuakeConfig) {
    let path = PathBuf::from(&conf.workspace);
    let defines = EntryDefines::from_path(&path.join(EntryPaths::entries_define()));

    let content = serde_json::to_string(&defines).unwrap();
    let out_path = PathBuf::from(DUMP_PATH).join("defines");

    fs::write(out_path, content).unwrap();
}

fn dump_entries_data(conf: &QuakeConfig) {
    let path = PathBuf::from(&conf.workspace);
    let defines = EntryDefines::from_path(&path.join(EntryPaths::entries_define()));

    for define in &defines.entries {
        let entry_type = &*define.entry_type;
        let define = defines
            .find(entry_type)
            .unwrap_or_else(|| panic!("lost entry define for: {:?}", &entry_type));
        let entry_path = path.join(&entry_type);

        let index = 1;
        let type_maps = define.to_field_type();
        let target_dir = PathBuf::from(DUMP_PATH).join("entry").join(entry_type);
        fs::create_dir_all(&target_dir).unwrap();
        for path in &Entrysets::scan_files(&*entry_path) {
            let content = Entrysets::file_to_json(&define, index, &type_maps, path).unwrap();

            let file = target_dir.join(EntryFile::file_prefix(index).to_string());

            fs::write(file, content.to_string()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use quake_core::QuakeConfig;

    use crate::page_dump;
    use crate::pagedump::DUMP_PATH;

    fn config() -> QuakeConfig {
        let path = PathBuf::from("_fixtures").join("demo_quake");
        QuakeConfig {
            workspace: path.display().to_string(),
            editor: "".to_string(),
            search_url: "http://127.0.0.1:7700".to_string(),
            server_location: "web".to_string(),
            port: 8000,
        }
    }

    #[test]
    fn should_dump_entries_define() {
        page_dump(config());
        let defines = PathBuf::from(DUMP_PATH).join("defines");
        assert!(defines.exists());

        let transflow = PathBuf::from(DUMP_PATH)
            .join("transflow")
            .join("script")
            .join("gen_code.js");
        assert!(transflow.exists());

        page_dump(config());
        let layout = PathBuf::from(DUMP_PATH).join("layout").join("dashboard");
        assert!(layout.exists());

        let todo_entry = PathBuf::from(DUMP_PATH)
            .join("entry")
            .join("todo")
            .join("0001");
        assert!(todo_entry.exists());
    }
}
