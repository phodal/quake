use std::error::Error;
use std::fs;
use std::path::PathBuf;

use rusqlite::{Connection, Row};
use rusqlite::types::ValueRef;

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::front_matter::FrontMatter;

pub fn dump_phodal_com() {
    let path = PathBuf::from("_fixtures").join("phodal.com");
    let db_name = "phodal.dev";
    let sql = "SELECT blog_blogpost.id, blog_blogpost.keywords_string, blog_blogpost.title, blog_blogpost.description, blog_blogpost.slug, blog_blogpost.content,
       auth_user.first_name, auth_user.last_name, auth_user.email
FROM blog_blogpost
         INNER JOIN auth_user
                    ON blog_blogpost.user_id = auth_user.id
";

    let _ = export_mezzanine(db_name, sql, path);
}

pub fn export_mezzanine(db_name: &str, sql: &str, path: PathBuf) -> Result<(), Box<dyn Error>> {
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
    use crate::mezzanine::dump_phodal_com;

    #[test]
    fn dump_test() {
        let _ = dump_phodal_com();
    }
}
