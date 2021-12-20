use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
use quake_core::markdown::md_processor::MdProcessor;
use quake_core::usecases::entrysets::Entrysets;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub fn generate_links(path: &Path) {
    let defines = EntryDefines::from_path(&path.join(EntryPaths::entries_define()));

    for define in &defines.entries {
        let entry_path = path.join(&define.entry_type);

        let mut _index = 1;
        for path in &Entrysets::scan_files(&*entry_path) {
            let string = fs::read_to_string(path).unwrap();

            let _ = MdProcessor::pagelinks(&string);
            // index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::usecases::pagelink_usecases::generate_links;
    use std::path::PathBuf;

    #[test]
    fn should_generate_links() {
        let path = PathBuf::from("_fixtures").join("demo_quake");
        generate_links(&path);
    }
}
