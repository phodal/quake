use std::fs::File;
use std::io::BufWriter;
use std::path;
use std::path::{Path, PathBuf};

use lopdf::Document;
use pdf_extract::{output_doc, print_metadata, HTMLOutput, OutputDev, PlainTextOutput, SVGOutput};

pub fn pdf_file_to_content(file: &Path) {
    let output_kind = "txt";

    let path = path::Path::new(&file);
    let filename = path.file_name().expect("expected a filename");

    let mut output_file = PathBuf::new();
    output_file.push(filename);
    output_file.set_extension(&output_kind);

    let mut output_file =
        BufWriter::new(File::create(output_file).expect("could not create output"));
    let doc = Document::load(path).unwrap();

    print_metadata(&doc);

    let mut output: Box<dyn OutputDev> = match output_kind {
        "txt" => Box::new(PlainTextOutput::new(
            &mut output_file as &mut dyn std::io::Write,
        )),
        "html" => Box::new(HTMLOutput::new(&mut output_file)),
        "svg" => Box::new(SVGOutput::new(&mut output_file)),
        _ => panic!(),
    };

    let _ = output_doc(&doc, output.as_mut());
}

#[cfg(test)]
mod tests {
    use crate::pdf_processor::pdf_file_to_content;
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        let file = PathBuf::from("_fixtures").join("pdf-test.pdf");
        pdf_file_to_content(&file);
    }
}
