use std::error::Error;
use std::path;
use std::path::Path;

use pdf_extract::extract_text;

use crate::process_engine::Processor;

#[derive(Default)]
pub struct PdfProcessor {}

impl Processor for PdfProcessor {
    fn content(&self, file: &Path) -> Result<String, Box<dyn Error>> {
        let path = path::Path::new(&file);
        let string = extract_text(path)?;
        Ok(string)
    }
}
