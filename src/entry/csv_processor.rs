use std::{fs, io};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use serde::Deserialize;
use serde_derive::Serialize;
use walkdir::{DirEntry, WalkDir};

use crate::CustomEntry;
use crate::entry::entry_file::EntryFile;

pub struct CsvProcessor {
    pub entry: CustomEntry,
}

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

impl CsvProcessor {
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

    pub fn write(_path: PathBuf, header: Vec<String>, body: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(io::stdout());

        wtr.write_record(&header)?;

        for column in body {
            wtr.write_record(&column)?;
        }

        wtr.flush()?;

        Ok(())
    }

    /// scan all entries files, and rebuild indexes
    pub fn rebuild(path: PathBuf) -> (Vec<String>, Vec<Vec<String>>) {
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

        let mut header: Vec<String> = vec![];
        header.push("id".to_string());

        let mut body: Vec<Vec<String>> = vec![];
        let mut has_first = false;

        let mut index = 1;
        for file in files {
            let string = fs::read_to_string(file).expect("cannot read file");

            let entry_file = EntryFile::from(&*string);
            let (mut first_header, column) = entry_file.header_column(index);

            if !has_first {
                header.append(&mut first_header);
                has_first = true;
            }

            body.push(column);
            index = index + 1;
        }

        (header, body)
    }

    /// update in column
    pub fn update_by_column() {}
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::entry::csv_processor::CsvProcessor;

    #[test]
    fn read_csv() {
        let buf = PathBuf::from("_fixtures").join("todo").join("entrysets.csv");
        match CsvProcessor::read(buf) {
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
        let source = PathBuf::from("_fixtures");
        let buf = PathBuf::from("_fixtures").join("todo");
        let map = CsvProcessor::rebuild(buf);
        let _ = CsvProcessor::write(source, map.0, map.1);
    }
}
