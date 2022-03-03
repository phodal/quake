use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::{panic, path};

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

        println!("processing file: {:}", path.display());

        let extract = panic::catch_unwind(|| extract_text(path));
        let text = match extract {
            Ok(t) => match t {
                Ok(text) => text,
                Err(err) => return Err(Box::new(err)),
            },
            Err(err) => {
                println!("{:?}", err);
                "".to_string()
            }
        };

        string.push_str(text.as_str());

        string = string.replace("\n\n", "<quake-br>").replace('\n', "");
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
