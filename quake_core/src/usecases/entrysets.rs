use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use json::{array, object, JsonValue};
use serde::Deserialize;
use serde_derive::Serialize;
use walkdir::{DirEntry, WalkDir};

use crate::entry::entry_file::EntryFile;
use crate::entry::EntryDefine;
use crate::helper::quake_time;
use crate::meta::MetaProperty;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct CsvTable {
    pub header: Vec<String>,
    pub body: Vec<Vec<String>>,
}

pub struct Entrysets {
    pub entry: EntryDefine,
}

impl Entrysets {
    pub fn read(path: PathBuf) -> Result<CsvTable, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = csv::ReaderBuilder::new().from_reader(file);

        let mut table = CsvTable::default();
        if let Ok(record) = rdr.headers() {
            for str in record {
                table.header.push(String::from(str))
            }
        }

        for result in rdr.records() {
            let record = result?;
            let mut row = vec![];
            for str in &record {
                row.push(String::from(str));
            }
            table.body.push(row);
        }

        Ok(table)
    }

    pub fn content_by_table(
        header: Vec<String>,
        body: Vec<Vec<String>>,
    ) -> Result<String, Box<dyn Error>> {
        header.len();
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(vec![]);

        wtr.write_record(&header).unwrap();

        for column in &body {
            if let Err(err) = wtr.write_record(&*column) {
                println!("{:?}", column);
                println!("parse csv column issue {:?}", &err);
            };
        }

        wtr.flush()?;

        Ok(String::from_utf8(wtr.into_inner()?)?)
    }

    /// scan all entries files, and rebuild indexes
    pub fn jsonify(path: &Path) -> Result<String, Box<dyn Error>> {
        let files = Self::scan_files(path);
        let mut index = 1;

        let mut entry_sets: Vec<EntryFile> = vec![];
        for file in files {
            let string = fs::read_to_string(&file)?;

            let mut entry_file = EntryFile::from(&*string, index)?;
            entry_file.name = file.file_name().unwrap().to_str().unwrap().to_string();

            entry_sets.push(entry_file);
            index += 1;
        }

        Ok(serde_json::to_string(&entry_sets)?)
    }

    /// format json from define
    pub fn jsonify_with_format_date(
        path: &Path,
        define: &EntryDefine,
    ) -> Result<JsonValue, Box<dyn Error>> {
        let files = Self::scan_files(path);
        let mut index = 1;

        let mut json: JsonValue = array![];
        let type_maps = define.to_field_type();
        for file in files {
            let mut element = object! {};
            let string = fs::read_to_string(&file)?;

            let mut entry_file = EntryFile::from(&*string, index)?;
            entry_file.name = (&file.file_name().unwrap().to_str().unwrap()).to_string();

            let mut error = "".to_string();
            let mut has_convert_date_issue = false;
            for (k, v) in &entry_file.properties {
                if let Some(MetaProperty::Date(_date)) = type_maps.get(k) {
                    let value = quake_time::replace_to_unix(v);
                    match value.parse::<usize>() {
                        Ok(time) => {
                            element[k.clone()] = time.into();
                            continue;
                        }
                        Err(err) => {
                            if !has_convert_date_issue {
                                error =
                                    format!("parse {:?} field: {:?},  error:{:?}", file, k, err);
                            }
                            has_convert_date_issue = true;
                        }
                    }
                }

                element[k.clone()] = v.clone().into();
            }

            if has_convert_date_issue {
                println!("{:?}", error);
            }

            element["id".to_string()] = entry_file.id.into();
            element["content".to_string()] = entry_file.content.into();
            element["type".to_string()] = define.entry_type.clone().into();

            json.push(element)?;

            index += 1;
        }

        Ok(json)
    }

    /// scan all entries files, and rebuild indexes
    pub fn rebuild(path: &Path) -> Result<CsvTable, Box<dyn Error>> {
        let files = Self::scan_files(path);

        let mut header: Vec<String> = vec!["id".to_string()];

        let mut body: Vec<Vec<String>> = vec![];
        let mut has_first = false;

        let mut index = 1;
        for file in files {
            let string = fs::read_to_string(&file)?;

            let mut entry_file = match EntryFile::from(&*string, index) {
                Ok(file) => file,
                Err(err) => {
                    println!("create entry file error: {:?}", file.display());
                    return Err(err);
                }
            };
            entry_file.name = file.file_name().unwrap().to_str().unwrap().to_string();

            let (mut first_header, column) = entry_file.header_column(index);

            if !has_first {
                header.append(&mut first_header);
                has_first = true;
            }

            body.push(column);
            index += 1;
        }

        Ok(CsvTable { header, body })
    }

    fn scan_files(path: &Path) -> Vec<PathBuf> {
        fn is_markdown(entry: &DirEntry) -> bool {
            entry
                .file_name()
                .to_str()
                .map(|s| s.ends_with(".md"))
                .unwrap_or(false)
        }

        let mut files = vec![];
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if is_markdown(&entry) {
                files.push(entry.into_path());
            }
        }

        files
    }

    pub fn generate(path: &Path) -> Result<(usize, String), Box<dyn Error>> {
        let map = match Entrysets::rebuild(path) {
            Ok(table) => table,
            Err(err) => {
                println!("path: {:?}, {:?}", path.display(), err);
                return Err(err);
            }
        };
        let table_len = map.body.len();
        let string = Entrysets::content_by_table(map.header, map.body)?;

        Ok((table_len, string))
    }

    pub fn define_from_csv(path_name: String, csv: PathBuf) -> Result<EntryDefine, Box<dyn Error>> {
        let mut define = EntryDefine::default();
        let table = Entrysets::read(csv)?;
        define.entry_type = path_name;
        for name in table.header {
            if name.eq("id") {
                continue;
            }

            let mut map = IndexMap::new();

            if name.contains("date") {
                map.insert(name, "Date".to_string());
            } else if name.eq("title") {
                map.insert(name, "Title".to_string());
            } else if name.eq("content") {
                map.insert(name, "Body".to_string());
            } else {
                map.insert(name, "String".to_string());
            }

            define.properties.push(map);
        }

        Ok(define)
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::EntryDefine;
    use std::path::PathBuf;

    use crate::usecases::entrysets::Entrysets;

    #[test]
    fn read_csv() {
        let buf = PathBuf::from("..")
            .join("examples")
            .join("todo")
            .join("entries.csv");

        match Entrysets::read(buf) {
            Ok(table) => {
                assert_eq!("id".to_string(), table.header[0]);
                assert_eq!("1".to_string(), table.body[0][0]);
            }
            Err(_err) => panic!(),
        }
    }

    #[test]
    fn test_define_from_csv() {
        let buf = PathBuf::from("..")
            .join("examples")
            .join("todo")
            .join("entries.csv");

        let define = Entrysets::define_from_csv("todo".to_string(), buf).unwrap();

        assert_eq!("todo", define.entry_type);

        assert_eq!("Title", define.properties[0].get("title").unwrap());
        assert_eq!("String", define.properties[1].get("author").unwrap());
        assert_eq!("Date", define.properties[2].get("created_date").unwrap());
        assert_eq!("Date", define.properties[3].get("updated_date").unwrap());
    }

    #[test]
    fn test_generate() {
        let buf = PathBuf::from("..").join("examples").join("todo");

        let (size, string) = Entrysets::generate(&buf).unwrap();

        assert_eq!(1, size);
        assert_eq!( "\"id\",\"title\",\"author\",\"created_date\",\"updated_date\"\n1,\"time support\",\"\",\"2021-11-24 19:14:10\",\"2021-11-24 19:14:10\"\n", string);
    }

    #[test]
    fn rebuild() {
        let buf = PathBuf::from("..").join("examples").join("todo");
        let map = Entrysets::rebuild(&buf).unwrap();
        match Entrysets::content_by_table(map.header, map.body) {
            Ok(some) => {
                println!("{}", some);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    #[test]
    fn jsonify_todo() {
        let buf = PathBuf::from("..").join("examples").join("todo");
        let json = Entrysets::jsonify(&buf).unwrap();

        #[cfg(not(windows))]
        assert_eq!(json, "[{\"title\":\"time support\",\"author\":\"\",\"content\":\"\",\"created_date\":\"2021-11-24 19:14:10\",\"updated_date\":\"2021-11-24 19:14:10\",\"id\":1,\"content\":\"\\n\\nahaha\\n\"}]");
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
    fn jsonify_todo_with_date() {
        let buf = PathBuf::from("..").join("examples").join("todo");
        let json = Entrysets::jsonify_with_format_date(&buf, &todo_define()).unwrap();

        #[cfg(not(windows))]
        assert_eq!(json.to_string(), "[{\"title\":\"time support\",\"author\":\"\",\"content\":\"\\n\\nahaha\\n\",\"created_date\":1637781250,\"updated_date\":1637781250,\"id\":1,\"type\":\"todo\"}]");
    }
}
