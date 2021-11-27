use std::error::Error;
use std::fs;
use std::path::PathBuf;

use rusqlite::{Connection, Row};
use rusqlite::types::ValueRef;

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::front_matter::FrontMatter;

pub fn export(db_name: &str, sql: &str, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db_name)?;
    let mut query = conn.prepare(sql)?;

    let mut rows = query.query([])?;

    let mut id: usize = 1;
    while let Some(row) = rows.next()? {
        write_file(&path, row,  id);
        id = id +1;
    };

    Ok(())
}

pub fn write_file(path: &PathBuf, row: &Row, id: usize) {
    let mut file = EntryFile::default();
    let mut matter = FrontMatter::default();
    let mut title = "".to_string();

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

            matter.fields.insert(name.to_string(), simple_escape(value));
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
