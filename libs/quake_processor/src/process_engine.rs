use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use crate::pdf_processor::PdfProcessor;

pub trait Processor {
    fn content(&self, file: &Path) -> Result<String, Box<dyn Error>>;
    fn meta_data(&self) -> HashMap<String, String>;
}

#[derive(Default)]
pub struct EmptyProcessor {}

impl Processor for EmptyProcessor {
    fn content(&self, _file: &Path) -> Result<String, Box<dyn Error>> {
        Ok("".to_string())
    }

    fn meta_data(&self) -> HashMap<String, String> {
        todo!()
    }
}

pub struct ProcessEngine {}

impl ProcessEngine {
    pub fn engine(text: &str) -> Box<dyn Processor> {
        let processor: Box<dyn Processor> = match text {
            "pdf" => Box::new(PdfProcessor::default()),
            _ => Box::new(EmptyProcessor::default()),
        };

        processor
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::process_engine::ProcessEngine;

    #[test]
    fn it_works() {
        let file = PathBuf::from("_fixtures").join("Test_PDF.pdf");
        let processor = ProcessEngine::engine("pdf");
        match processor.content(&file) {
            Err(err) => {
                info!("{:?}", err);
                panic!();
            }
            Ok(some) => {
                info!("{:?}", some);
            }
        }
    }
}
