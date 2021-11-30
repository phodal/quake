use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use quake_core::entry::{EntryDefine, EntryInfo, FrontMatter};
use quake_core::entry::entry_file::EntryFile;

use crate::action::entry_factory;
use crate::action::entry_paths::EntryPaths;
use crate::action::entrysets::Entrysets;
use crate::errors::QuakeError;
use crate::helper::file_process;

pub fn find_entry_define(paths: &EntryPaths, target_entry: &String) -> EntryDefine {
    let entries: Vec<EntryDefine> = entry_factory::entries_define_from_path(&paths.entries_define).into_iter()
        .filter(|define| {
            define.entry_type.eq(target_entry)
        })
        .collect();

    let entries_define = if entries.len() == 0 {
        EntryDefine::default()
    } else {
        entries[0].clone()
    };
    entries_define
}

pub fn sync_in_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let (size, content) = Entrysets::generate(&paths.base)?;
    fs::write(&paths.entries, content)?;

    update_entry_info(&paths.entries_info, &mut EntryInfo {
        index: size
    });

    Ok(())
}

pub fn update_entry_info(entry_info_path: &PathBuf, entry_info: &mut EntryInfo) {
    let result = serde_yaml::to_string(&entry_info).expect("cannot convert to yaml");
    fs::write(&entry_info_path, result).expect("cannot write to file");
}

pub fn create_entry(quake_path: &String, entry_type: &String, entry_text: &String) -> Result<(PathBuf, EntryFile), Box<dyn Error>> {
    let paths = EntryPaths::init(quake_path, entry_type);
    let entries_define = find_entry_define(&paths, entry_type);
    let mut entry_info = entry_factory::entry_info_from_path(&paths.entries_info);

    let new_md_file = file_process::file_name(entry_info.index + 1, entry_text.as_str());
    let mut target_path = paths.base.join(new_md_file);
    File::create(&target_path)?;

    let entry_file = create_entry_file(&entries_define, &mut target_path, entry_text.to_string());

    entry_info.inc();
    update_entry_info(&paths.entries_info, &mut entry_info);

    Ok((target_path, entry_file))
}

pub fn create_entry_file(entry_define: &EntryDefine, target_file: &mut PathBuf, entry_text: String) -> EntryFile {
    let mut entry_file = EntryFile::default();
    let init_map = entry_define.create_title_and_date(entry_text);
    entry_file.front_matter = FrontMatter { fields: entry_define.merge(init_map) };

    fs::write(&target_file, entry_file.to_string()).expect("cannot write to file");

    entry_file
}

pub fn find_entry_path(entry_path: PathBuf, entry_type: &String, index: usize) -> Result<PathBuf, Box<QuakeError>> {
    #[allow(unused_assignments)]
    let mut target_file = PathBuf::new();

    let prefix = file_process::file_prefix(index);
    let vec = file_process::filter_by_prefix(entry_path, prefix);
    if vec.len() > 0 {
        target_file = vec[0].clone();
    } else {
        return Err(Box::new(QuakeError(format!("cannot find {:} file {:}", entry_type, index))));
    }

    Ok(target_file)
}

pub fn update_entry_fields(type_path: PathBuf, entry_type: &str, index_id: usize, map: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let entry_path = find_entry_path(type_path, &entry_type.to_string(), index_id)?;
    let string = fs::read_to_string(&entry_path)?;
    let mut entry_file = EntryFile::from(string.as_str())?;

    for (key, value) in map {
        entry_file.update_field(key, value);
    }
    fs::write(&entry_path, entry_file.to_string())?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    use rocket::form::validate::Contains;

    use quake_core::entry::entry_file::EntryFile;

    use crate::action::entry_usecases::{find_entry_path, update_entry_fields};

    #[test]
    fn update_entry_title() {
        let yiki_path = PathBuf::from("_fixtures").join("yiki");
        let entry_type = "yiwi";
        let index_id = 1;

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("title".to_string(), "this is a test".to_string());

        update_entry_fields(yiki_path.clone(), &entry_type, index_id, map).unwrap();

        let entry_path = find_entry_path(yiki_path, &entry_type.to_string(), index_id).unwrap();
        let string = fs::read_to_string(&entry_path).unwrap();
        assert!(string.contains("this is a test".to_string().as_str()));

        let string = fs::read_to_string(&entry_path).unwrap();
        let mut entry_file = EntryFile::from(string.as_str()).unwrap();
        entry_file.update_field("title".to_string(), "概念知识容量表".to_string());
        fs::write(&entry_path, entry_file.to_string()).unwrap();

        let string = fs::read_to_string(&entry_path).unwrap();
        assert!(string.contains("概念知识容量表"));
    }
}
