use std::path::PathBuf;
use rusqlite::Row;
use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::front_matter::FrontMatter;
use rusqlite::types::ValueRef;
use std::fs;

pub fn write_file(path: &PathBuf, row: &Row) {
    let mut file = EntryFile::default();
    let mut matter = FrontMatter::default();
    let mut title = "".to_string();
    let mut id: usize = 0;

    for (index, name) in row.column_names().iter().enumerate() {
        let value: String = match row.get_ref(index).unwrap() {
            ValueRef::Null => { "".to_string() }
            ValueRef::Integer(int) => { int.to_string() }
            ValueRef::Real(real) => { real.to_string() }
            ValueRef::Text(text) => { std::str::from_utf8(text).unwrap().to_string() }
            ValueRef::Blob(bool) => { std::str::from_utf8(bool).unwrap().to_string() }
        };

        let name = name.to_string();
        if name.eq("content") {
            file.content.push_str("\n");
            file.content.push_str("\n");
            file.content.push_str(&*value);
        } else {
            if name.eq("title") {
                title = value.clone();
            }

            matter.fields.insert(name.to_string(), mezzanine::simple_escape(value));
        }
    }

    file.name = EntryFile::file_name(id, title.as_str());
    file.front_matter = matter;

    match fs::write(path.join(file.name.clone()), file.to_string()) {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", file.name.clone());
            println!("{:?}", err);
        }
    }
}

fn simple_escape(value: String) -> String {
    format!("{:?}", value
        .replace(" ", " ")
        .replace("", " ")
        .replace("", " ")
        .replace("", " ")
        .replace("​", " ")
    )
}

