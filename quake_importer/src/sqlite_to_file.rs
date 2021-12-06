use std::error::Error;
use std::fs;
use std::path::PathBuf;

use rusqlite::types::ValueRef;
use rusqlite::{Connection, Row};

use quake_core::entry::entry_file::EntryFile;

pub fn export(db_name: &str, sql: &str, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db_name)?;
    let mut query = conn.prepare(sql)?;

    let mut rows = query.query([])?;

    let mut id: usize = 1;
    while let Some(row) = rows.next()? {
        write_file(&path, row, id);
        id = id + 1;
    }

    Ok(())
}

pub fn write_file(path: &PathBuf, row: &Row, id: usize) {
    let mut file = EntryFile::default();
    let mut title = "".to_string();

    for (index, name) in row.column_names().iter().enumerate() {
        let value: String = match row.get_ref(index).unwrap() {
            ValueRef::Null => "".to_string(),
            ValueRef::Integer(int) => int.to_string(),
            ValueRef::Real(real) => real.to_string(),
            ValueRef::Text(text) => std::str::from_utf8(text).unwrap().to_string(),
            ValueRef::Blob(bool) => std::str::from_utf8(bool).unwrap().to_string(),
        };

        let name = name.to_string();
        if name.eq("content") {
            file.content = "\n\n".to_string();
            file.content.push_str(&*value);
        } else {
            if name.eq("title") {
                title = value.clone();
            }

            file.add_field(name.as_str(), simple_escape(value).as_str());
        }
    }

    file.name = EntryFile::file_name(id, title.as_str());

    match fs::write(path.join(file.name.clone()), file.to_string()) {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", file.name.clone());
            println!("{:?}", err);
        }
    }
}

fn simple_escape(value: String) -> String {
    format!(
        "{:?}",
        value
            .replace(" ", " ")
            .replace("", " ")
            .replace("", " ")
            .replace("", " ")
            .replace("​", " ")
    )
}
