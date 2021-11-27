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

fn export_mezzanine(db_name: &str, sql: &str, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let _ = fs::create_dir(&path);

    let conn = Connection::open(db_name)?;
    let mut query = conn.prepare(sql)?;

    let mut rows = query.query([])?;

    while let Some(row) = rows.next()? {
        write_file(&path, row);
    };

    Ok(())
}

fn write_file(path: &PathBuf, row: &Row) {
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

            if name.eq("id") {
                id = value.parse().unwrap();
                matter.fields.insert(name.to_string(), id.to_string());
            } else {
                let string = simple_escape(value);
                matter.fields.insert(name.to_string(),
                                     string
                );
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


#[cfg(test)]
mod tests {
    use crate::mezzanine::dump_phodal_com;

    #[test]
    fn escape_test() {
        let origin = "Mac OS 安装GNU命令行工具";
        let esc = origin.replace(" ", " ");

        println!("{}", esc);
    }

    #[test]
    fn dump_test() {
        let _ = dump_phodal_com();
    }
}
