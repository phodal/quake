use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use indexmap::IndexMap;
use serde::Deserialize;
use serde_derive::Serialize;
use walkdir::{DirEntry, WalkDir};

use crate::entry::entry_file::EntryFile;
use crate::entry::EntryDefine;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CsvTable {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Default for CsvTable {
    fn default() -> Self {
        CsvTable {
            header: vec![],
            rows: vec![],
        }
    }
}

pub struct Entrysets {
    pub entry: EntryDefine,
}

impl Entrysets {
    pub fn read(path: PathBuf) -> Result<CsvTable, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = csv::ReaderBuilder::new().from_reader(file);

        let mut table = CsvTable::default();
        for record in rdr.headers() {
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
            table.rows.push(row);
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
    pub fn jsonify(path: &PathBuf) -> Result<String, Box<dyn Error>> {
        let files = Self::scan_files(path);
        let mut index = 1;

        let mut entry_sets: Vec<EntryFile> = vec![];
        for file in files {
            let string = fs::read_to_string(&file)?;

            let mut entry_file = EntryFile::from(&*string, index)?;
            entry_file.name = format!("{}", file.file_name().unwrap().to_str().unwrap());

            entry_sets.push(entry_file);
            index = index + 1;
        }

        Ok(serde_json::to_string(&entry_sets)?)
    }

    /// scan all entries files, and rebuild indexes
    pub fn rebuild(path: &PathBuf) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>> {
        let files = Self::scan_files(path);

        let mut header: Vec<String> = vec![];
        header.push("id".to_string());

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
            entry_file.name = format!("{}", file.file_name().unwrap().to_str().unwrap());

            let (mut first_header, column) = entry_file.header_column(index);

            if !has_first {
                header.append(&mut first_header);
                has_first = true;
            }

            body.push(column);
            index = index + 1;
        }

        Ok((header, body))
    }

    fn scan_files(path: &PathBuf) -> Vec<PathBuf> {
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

    pub fn generate(path: &PathBuf) -> Result<(usize, String), Box<dyn Error>> {
        let map = match Entrysets::rebuild(&path) {
            Ok((header, body)) => (header, body),
            Err(err) => {
                println!("path: {:?}, {:?}", path.display(), err);
                return Err(err);
            }
        };
        let table_len = map.1.len();
        let string = Entrysets::content_by_table(map.0, map.1)?;

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

            define.fields.push(map);
        }

        Ok(define)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::usecases::entrysets::Entrysets;

    #[test]
    fn read_csv() {
        let buf = PathBuf::from("examples").join("todo").join("entries.csv");
        match Entrysets::read(buf) {
            Ok(_table) => {
                // println!("{:?}", table);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    #[test]
    fn rebuild() {
        let buf = PathBuf::from("..").join("examples").join("todo");
        let map = Entrysets::rebuild(&buf).unwrap();
        match Entrysets::content_by_table(map.0, map.1) {
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
}
