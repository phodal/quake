use std::fs;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use serde::Deserialize;
use serde_derive::Serialize;
use walkdir::{DirEntry, WalkDir};

use quake_core::entry::entry_define::EntryDefine;
use quake_core::entry::entry_file::EntryFile;

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

pub struct EntrysetsCsv {
    pub entry: EntryDefine,
}

impl EntrysetsCsv {
    pub fn read(path: PathBuf) -> Result<CsvTable, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = csv::ReaderBuilder::new()
            .from_reader(file);

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

    pub fn content_by_table(header: Vec<String>, body: Vec<Vec<String>>) -> Result<String, Box<dyn Error>> {
        header.len();
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(vec![]);

        wtr.write_record(&header)?;

        for column in body {
            wtr.write_record(&column)?;
        }

        wtr.flush()?;

        Ok(String::from_utf8(wtr.into_inner()?)?)
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

            let mut entry_file = EntryFile::from(&*string);
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
            entry.file_name()
                .to_str()
                .map(|s| s.ends_with(".md"))
                .unwrap_or(false)
        }

        let mut files = vec![];
        for entry in WalkDir::new(path).into_iter()
            .filter_map(|e| e.ok()) {
            if is_markdown(&entry) {
                files.push(entry.into_path());
            }
        }
        files
    }

    pub fn generate(path: &PathBuf) -> Result<(usize, String), Box<dyn Error>> {
        let map = EntrysetsCsv::rebuild(&path)?;
        let table_len = map.1.len();
        let string = EntrysetsCsv::content_by_table(map.0, map.1)?;

        Ok((table_len, string))
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::action::entrysets_csv::EntrysetsCsv;

    #[test]
    fn read_csv() {
        let buf = PathBuf::from("_fixtures").join("todo").join("entries.csv");
        match EntrysetsCsv::read(buf) {
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
        let buf = PathBuf::from("_fixtures").join("todo");
        let map = EntrysetsCsv::rebuild(&buf).unwrap();
        match EntrysetsCsv::content_by_table(map.0, map.1) {
            Ok(some) => {
                println!("{}", some);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
