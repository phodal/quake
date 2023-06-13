use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use crate::entry::entry_file::EntryFile;
use crate::entry::entry_paths::EntryPaths;
use crate::entry::{entry_node_info, EntryDefine, EntryNodeInfo};
use crate::errors::QuakeError;
use crate::helper::{date_now, file_filter};
use crate::usecases::entry_define_usecases;
use crate::usecases::entrysets::Entrysets;

/// generate entries.csv from by paths
pub fn sync_in_path(paths: &EntryPaths) -> Result<(), Box<dyn Error>> {
    let (size, content) = Entrysets::generate_csv(&paths.entry_path)?;

    if size > 0 {
        fs::write(&paths.entries_csv, content)?;
        entry_node_info::save_entry_info(&paths.entry_node_info, &mut EntryNodeInfo::new(size));
    }

    Ok(())
}

/// create entry by `path`, `type`, `text`
/// process:
/// 1. looking for entry define file
/// 2. create entry file
/// 3. update entry node info index
///
pub fn create_entry(
    quake_path: &str,
    entry_type: &str,
    entry_text: &str,
) -> Result<(PathBuf, EntryFile), Box<dyn Error>> {
    let paths = EntryPaths::init(quake_path, entry_type);
    let entries_define =
        entry_define_usecases::find_entry_define(entry_type, &paths.entries_define);
    let mut entry_info = entry_node_info::entry_info_from_path(&paths.entry_node_info);

    let new_index = entry_info.index + 1;
    let new_md_file = EntryFile::file_name(new_index, entry_text);

    let mut target_path = paths.entry_path.join(new_md_file);
    File::create(&target_path)?;

    let mut entry_file = create_entry_file(&entries_define, &mut target_path, entry_text);
    entry_file.id = new_index;

    entry_info.inc();
    entry_node_info::save_entry_info(&paths.entry_node_info, &mut entry_info);

    Ok((target_path, entry_file))
}

/// create really entry file
pub fn create_entry_file(
    entry_define: &EntryDefine,
    target_file: &mut PathBuf,
    entry_text: &str,
) -> EntryFile {
    let mut entry_file = EntryFile::default();
    entry_file.set_properties(entry_define.create_default_properties(entry_text));
    fs::write(target_file, entry_file.to_string()).expect("cannot write to file");
    entry_file
}

pub fn find_entry_path(
    entry_path: PathBuf,
    entry_type: &str,
    index: usize,
) -> Result<PathBuf, Box<QuakeError>> {
    #[allow(unused_assignments)]
    let mut target_file = PathBuf::new();

    let prefix = EntryFile::file_prefix(index);
    let vec = file_filter::filter_by_prefix(entry_path, prefix);
    if !vec.is_empty() {
        target_file = vec[0].clone();
    } else {
        return Err(Box::new(QuakeError(format!(
            "cannot find {:} file {:}",
            entry_type, index
        ))));
    }

    Ok(target_file)
}

pub fn update_entry_properties(
    type_path: PathBuf,
    entry_type: &str,
    index_id: usize,
    update_map: &HashMap<String, String>,
) -> Result<(PathBuf, EntryFile), Box<dyn Error>> {
    let entry_path = find_entry_path(type_path, entry_type, index_id)?;
    let string = fs::read_to_string(&entry_path)?;
    let mut entry_file = EntryFile::from(string.as_str(), index_id)?;

    for (key, value) in update_map {
        if key != "content" {
            entry_file.update_property(key, &format!("{:?}", value));
        }
    }

    if let Some(_val) = entry_file.properties.get("updated_date") {
        entry_file.update_property("updated_date", &date_now())
    }

    if let Some(val) = update_map.get("content") {
        entry_file.update_content(val);
    }

    fs::write(&entry_path, entry_file.to_string())?;

    Ok((entry_path, entry_file))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    use crate::entry::entry_file::EntryFile;
    use crate::usecases::entry_usecases::{create_entry, find_entry_path, update_entry_properties};

    #[test]
    fn create_todo_entry() {
        let quake_path = PathBuf::from("..").join("_fixtures").join("demo_quake");
        let path = quake_path.display().to_string();

        let result = create_entry(
            &path,
            &"create_todo_entry".to_string(),
            &"hello, world".to_string(),
        );

        match result {
            Ok((path, file)) => {
                assert!(path.display().to_string().contains("0001-hello-world.md"));
                assert_eq!("hello, world", file.property("title").unwrap())
            }
            Err(_err) => {
                assert!(false)
            }
        }

        let _ = fs::remove_dir_all(quake_path.join("create_todo_entry")).unwrap();
    }

    #[test]
    fn update_entry_title() {
        let yiki_path = PathBuf::from("..").join("examples").join("yiki");
        let entry_type = "yiwi";
        let index_id = 1;

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("title".to_string(), "this is a test".to_string());

        update_entry_properties(yiki_path.clone(), &entry_type, index_id, &map).unwrap();

        let entry_path = find_entry_path(yiki_path, &entry_type.to_string(), index_id).unwrap();
        let string = fs::read_to_string(&entry_path).unwrap();
        assert!(string.contains("this is a test".to_string().as_str()));

        let string = fs::read_to_string(&entry_path).unwrap();
        let mut entry_file = EntryFile::from(string.as_str(), index_id).unwrap();
        entry_file.update_property(&"title".to_string(), &"概念知识容量表".to_string());
        // reset time
        entry_file.update_property(
            &"updated_date".to_string(),
            &"2021-11-25 10:14:26".to_string(),
        );
        fs::write(&entry_path, entry_file.to_string()).unwrap();

        let string = fs::read_to_string(&entry_path).unwrap();
        assert!(string.contains("概念知识容量表"));
    }

    #[test]
    fn update_entry_content() {
        let yiki_path = PathBuf::from("..").join("examples").join("yiki");
        let entry_type = "yiwi";
        let index_id = 2;

        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("content".to_string(), "this is a content".to_string());

        update_entry_properties(yiki_path.clone(), &entry_type, index_id, &map).unwrap();

        let entry_path = find_entry_path(yiki_path, &entry_type.to_string(), index_id).unwrap();
        let string = fs::read_to_string(&entry_path).unwrap();
        assert!(string.contains("this is a content".to_string().as_str()));

        let string = fs::read_to_string(&entry_path).unwrap();
        let mut entry_file = EntryFile::from(string.as_str(), index_id).unwrap();
        entry_file.update_content(&"允许自定义字段\n".to_string());
        // reset time
        entry_file.update_property(
            &"updated_date".to_string(),
            &"2021-12-01 11:08:40".to_string(),
        );
        fs::write(&entry_path, entry_file.to_string()).unwrap();

        let string = fs::read_to_string(&entry_path).unwrap();
        assert!(string.contains("允许自定义字段\n"));
    }
}
