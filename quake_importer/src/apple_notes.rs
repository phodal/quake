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
 LastModified as updated_date, Data as content, User as author
  from Notes
";

    match export_apple_notes(db_path, sql, path) {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

pub fn export_apple_notes(db_name: &str, sql: &str, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let _ = fs::create_dir(&path);

    let conn = Connection::open(db_name)?;
    let mut query = conn.prepare(sql)?;

    let mut rows = query.query([])?;

    while let Some(row) = rows.next()? {
        crate::sql_to_file::write_file(&path, row);
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
