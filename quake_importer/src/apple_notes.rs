use std::error::Error;
use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use rusqlite::types::ValueRef;
use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::front_matter::FrontMatter;

/// refs: https://www.swiftforensics.com/2018/02/reading-notes-database-on-macos.html
pub fn dump_apple_notes(db_path: &str) {
    let path = PathBuf::from("..").join("_fixtures").join("notes");
    let sql = "
SELECT ID as id, Title as title, Snippet as description, Folder as category, Created as created_date,
 LastModified as updated_date, Data as content, User from Notes
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

    let mut id: usize = 0;

    while let Some(row) = rows.next()? {
        let mut file = EntryFile::default();
        let mut matter = FrontMatter::default();
        let mut title = "".to_string();

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
            if name.eq("content") {
                file.content.push_str("\n");
                file.content.push_str("\n");
                file.content.push_str(&*value);
            } else {
                if name.eq("title") {
                    title = value.clone();
                }

                if name.eq("id") {
                    id = id + 1;
                    matter.fields.insert(name.to_string(), id.to_string());
                } else {
                    matter.fields.insert(name.to_string(), format!("{:?}", value));
                }
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
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::apple_notes::dump_apple_notes;

    #[ignore]
    #[test]
    fn dump_notes() {
        let db_path = "../dbs/mac_apt.db";
        let _ = dump_apple_notes(db_path);
    }
}
