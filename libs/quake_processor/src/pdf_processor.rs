use std::collections::HashMap;
use std::error::Error;
use std::path;
use std::path::Path;

use pdf_extract::extract_text;

use crate::process_engine::Processor;

#[derive(Default, Clone)]
pub struct PdfProcessor {
    meta_data: HashMap<String, String>,
}

impl Processor for PdfProcessor {
    fn content(&self, file: &Path) -> Result<String, Box<dyn Error>> {
        let path = path::Path::new(&file);

        let mut string = String::new();
        string.push_str("<quake-br>");
        string.push_str(extract_text(path)?.as_str());

        string = string.replace("\n\n", "<quake-br>").replace("\n", "");
        string = string.replace("<quake-br>", "\n\n");

        Ok(string)
    }

    fn meta_data(&self) -> HashMap<String, String> {
        self.meta_data.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_parse_references() {}
}
