#[cfg(test)]
mod tests {
    extern crate lopdf;
    extern crate pdf_extract;

    use lopdf::*;
    use pdf_extract::*;
    use std::fs::File;
    use std::io::BufWriter;
    use std::path;
    use std::path::PathBuf;

    #[ignore]
    #[test]
    fn it_works() {
        let file = PathBuf::from("_fixtures").join("samples.pdf");

        let output_kind = "txt";
        println!("{:?}", file);
        let path = path::Path::new(&file);
        let filename = path.file_name().expect("expected a filename");
        let mut output_file = PathBuf::new();

        output_file.push(filename);
        output_file.set_extension(&output_kind);

        let mut output_file =
            BufWriter::new(File::create(output_file).expect("could not create output"));
        let doc = Document::load(path).unwrap();

        print_metadata(&doc);

        println!("{:?}", &doc.trailer);

        let mut output: Box<dyn OutputDev> = match output_kind.as_ref() {
            "txt" => Box::new(PlainTextOutput::new(
                &mut output_file as &mut dyn std::io::Write,
            )),
            "html" => Box::new(HTMLOutput::new(&mut output_file)),
            "svg" => Box::new(SVGOutput::new(&mut output_file)),
            _ => panic!(),
        };

        let _ = output_doc(&doc, output.as_mut());
    }
}
