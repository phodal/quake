use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;
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

    pub fn write() -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b',')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(io::stdout());

        wtr.write_record(&[
            "City",
            "State",
        ])?;
        wtr.write_record(&[
            "Davidsons Landing",
            "AK",
        ])?;

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
    use std::path::PathBuf;
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
        let _= EntriesCsvProcessor::write();
    }
}
