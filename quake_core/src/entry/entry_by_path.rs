use std::error::Error;
use std::fs;
use std::path::Path;

use crate::entry::entry_file::EntryFile;
use crate::entry::entry_paths::EntryPaths;
use crate::entry::{EntryDefine, EntryDefines};
use crate::errors::QuakeError;
use crate::helper::file_filter::type_from_md_path;
use crate::helper::quake_time;
use crate::meta::MetaProperty;

pub fn entry_file_dump(
    path: &Path,
    workspace: &Path,
) -> Result<(String, EntryFile), Box<dyn Error>> {
    let entry_type = type_from_md_path(path).ok_or("")?;
    let file_name = path.file_name().ok_or("")?;

    if file_name.is_empty() || entry_type.is_empty() {
        return Err(Box::new(QuakeError(format!(
            "empty type {:?} or file_name {:?}",
            entry_type, file_name
        ))));
    }

    let id = EntryFile::id_from_name(file_name.to_str().unwrap().to_string().as_str())?;
    let content = fs::read_to_string(&path)?;

    let mut file = EntryFile::from(content.as_str(), id)?;
    let defines = EntryDefines::from_path(&*workspace.join(EntryPaths::entries_define()));

    if let Some(define) = defines.find(&*entry_type) {
        for (key, prop) in define.to_field_type() {
            if let MetaProperty::Date(_date) = prop {
                let text = &*file.property(&key).unwrap();
                let value = quake_time::string_date_to_unix(text);
                file.update_property(&key, &value);
            }
        }

        file.add_property("type", entry_type.clone());
    }

    Ok((entry_type, file))
}

pub fn entry_file_by_path(path: &Path, define: &EntryDefine) -> Result<EntryFile, Box<dyn Error>> {
    let file_name = path.file_name().ok_or("")?;

    if file_name.is_empty() {
        return Err(Box::new(QuakeError(format!("file_name {:?}", file_name))));
    }

    let id = EntryFile::id_from_name(file_name.to_str().unwrap().to_string().as_str())?;
    let content = fs::read_to_string(&path)?;

    let mut file = EntryFile::from(content.as_str(), id)?;

    for (key, prop) in define.to_field_type() {
        if let MetaProperty::Date(_date) = prop {
            let text = &*file.property(&key).unwrap();
            let value = quake_time::string_date_to_unix(text);
            file.update_property(&key, &value);
        }

        file.add_property("type", define.entry_type.clone());
    }

    Ok(file)
}

#[cfg(test)]
mod tests {
    use crate::entry::entry_by_path::{entry_file_by_path, entry_file_dump};
    use crate::entry::EntryDefine;
    use std::path::PathBuf;

    #[test]
    fn entry_files_by_path_workspace() {
        let workspace = PathBuf::from("..").join("examples");
        let buf = workspace.join("todo").join("0001-time-support.md");

        let (typ, file) = entry_file_dump(&buf, &workspace).unwrap();
        assert_eq!(typ, "todo".to_string());
        assert_eq!(1, file.id);
        assert_eq!("1637781250", file.property("created_date").unwrap());
    }

    fn todo_define() -> EntryDefine {
        let yaml = "
- type: todo
  display: Todo
  properties:
    - title: Title
    - author: Author
    - created_date: Date
    - updated_date: Date
";

        let entries: Vec<EntryDefine> = serde_yaml::from_str(yaml).unwrap();
        entries[0].clone()
    }

    #[test]
    fn test_entry_file_by_path() {
        let workspace = PathBuf::from("..").join("examples");
        let buf = workspace.join("todo").join("0001-time-support.md");

        let file = entry_file_by_path(&buf, &todo_define()).unwrap();
        assert_eq!(1, file.id);
        assert_eq!("1637781250", file.property("created_date").unwrap());
    }
}
