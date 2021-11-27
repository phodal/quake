use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use rusqlite::Connection;
use rusqlite::types::ValueRef;

/// refs: https://www.swiftforensics.com/2018/02/reading-notes-database-on-macos.html
pub fn dump_apple_notes() {
    let path = PathBuf::from("_fixtures").join("phodal.com");
    let db_name = "../NoteStore.sqlite";
    let sql = "
SELECT n.Z_PK, n.ZNOTE as note_id, n.ZDATA as data,
       c3.ZFILESIZE,
       c4.ZFILENAME, c4.ZIDENTIFIER as att_uuid,
       c1.ZTITLE1 as title, c1.ZSNIPPET as snippet, c1.ZIDENTIFIER as noteID,
       c1.ZCREATIONDATE1 as created, c1.ZLASTVIEWEDMODIFICATIONDATE, c1.ZMODIFICATIONDATE1 as modified,
       c2.ZACCOUNT3, c2.ZTITLE2 as folderName, c2.ZIDENTIFIER as folderID,
       c5.ZNAME as acc_name, c5.ZIDENTIFIER as acc_identifier, c5.ZACCOUNTTYPE
FROM ZICNOTEDATA as n
         LEFT JOIN ZICCLOUDSYNCINGOBJECT as c1 ON c1.ZNOTEDATA = n.Z_PK
         LEFT JOIN ZICCLOUDSYNCINGOBJECT as c2 ON c2.Z_PK = c1.ZFOLDER
         LEFT JOIN ZICCLOUDSYNCINGOBJECT as c3 ON c3.ZNOTE= n.ZNOTE
         LEFT JOIN ZICCLOUDSYNCINGOBJECT as c4 ON c4.ZATTACHMENT1= c3.Z_PK
         LEFT JOIN ZICCLOUDSYNCINGOBJECT as c5 ON c5.Z_PK = c1.ZACCOUNT2
ORDER BY note_id
";

    match export_apple_notes(db_name, sql, path) {
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
                    if blob.len() <= 0 {
                        "".to_string()
                    } else {
                        let demo = format!("{:?}", blob);
                        let mut d = GzDecoder::new(blob);
                        let mut s = String::new();
                        match d.read_to_string(&mut s) {
                            Ok(so) => {}
                            Err(err) => {
                                println!("{:?}", err);
                            }
                        }
                        s
                    }
                }
            };

            let name = name.to_string();
            if name == "data" {
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
        let _ = dump_apple_notes();
    }
}
