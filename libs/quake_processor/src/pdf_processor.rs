use std::error::Error;
use std::path;
use std::path::Path;

use pdf_extract::extract_text;

pub fn pdf_file_to_content(file: &Path) -> Result<String, Box<dyn Error>> {
    let path = path::Path::new(&file);
    let string = extract_text(path)?;
    Ok(string)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::pdf_processor::pdf_file_to_content;

    #[test]
    fn it_works() {
        let file = PathBuf::from("_fixtures").join("Test_PDF.pdf");
        match pdf_file_to_content(&file) {
            Err(err) => {
                println!("{:?}", err);
                panic!();
            }
            Ok(some) => {
                println!("{:?}", some);
            }
        }
    }
}
