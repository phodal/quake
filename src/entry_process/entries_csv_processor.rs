use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use quake_core::model::CustomType;

use crate::CustomEntry;

pub struct EntriesCsvProcessor {
    pub entry: CustomEntry,
}

impl EntriesCsvProcessor {
    pub fn read(path: PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = csv::ReaderBuilder::new()
            .from_reader(file);
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }

        Ok(())
    }

    pub fn write(_path: PathBuf, values: Vec<CustomType>) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(io::stdout());

        let mut headers = vec![];
        for (key, _) in &values[0].fields {
            headers.push(key);
        }
        wtr.write_record(&headers)?;

        for field in values {
            let mut records = vec![];
            for (_key, field) in field.fields {
                records.push(format!("{}", field));
            }

            wtr.write_record(&records)?;
        }

        wtr.flush()?;

        Ok(())
    }

    /// scan all entries files, and rebuild indexes
    pub fn rebuild() {}

    /// update in column
    pub fn update_by_column() {}
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use quake_core::model::CustomType;

    use crate::entry_process::entries_csv_processor::EntriesCsvProcessor;

    #[test]
    fn read_csv() {
        let buf = PathBuf::from("_fixtures").join("todo").join("entrysets.csv");
        match EntriesCsvProcessor::read(buf) {
            Ok(_) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    #[test]
    fn write_csv() {
        let buf = PathBuf::from("samples");

        let mut map = HashMap::new();
        map.insert("title".to_string(), "Title".to_string());
        map.insert("keywords".to_string(), "#tag".to_string());

        let custom_type = CustomType::from(map);
        let mut values = vec![];
        values.push(custom_type);

        let _ = EntriesCsvProcessor::write(buf, values);
    }
}
