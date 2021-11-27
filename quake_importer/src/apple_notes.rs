use std::error::Error;
use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use rusqlite::types::ValueRef;

/// refs: https://www.swiftforensics.com/2018/02/reading-notes-database-on-macos.html
pub fn dump_apple_notes(db_path: &str) {
    let path = PathBuf::from("_fixtures").join("notes");
    let sql = "
SELECT Title, Snippet, Folder, LastModified, Data as content, User from Notes
";

    match export_apple_notes(db_path, sql, path) {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn export_apple_notes(db_name: &str, sql: &str, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let _ = fs::create_dir(&path);

    let conn = Connection::open(db_name)?;
    let mut query = conn.prepare(sql)?;

    let mut rows = query.query([])?;

    while let Some(row) = rows.next()? {
        for (index, name) in row.column_names().iter().enumerate() {
            let value: String = match row.get_ref(index).unwrap() {
                ValueRef::Null => {
                    "".to_string()
                }
                ValueRef::Integer(int) => { int.to_string() }
                ValueRef::Real(real) => { real.to_string() }
                ValueRef::Text(text) => { std::str::from_utf8(text).unwrap().to_string() }
                ValueRef::Blob(blob) => {
                    std::str::from_utf8(blob).unwrap().to_string()
                }
            };

            let name = name.to_string();
            if name == "content" {
                println!("{:?}: {:?}", name, value);
            }
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::apple_notes::dump_apple_notes;

    #[test]
    fn dump_notes() {
        let db_path = "../dbs/mac_apt.db";
        let _ = dump_apple_notes(db_path);
    }
}
