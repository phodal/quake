use std::error::Error;
use std::fs;
use std::path::PathBuf;
use clap::Parser;
use rusqlite::{Connection, Row};
use rusqlite::types::ValueRef;
use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::front_matter::FrontMatter;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    /// custom SQLite 3 to fields
    #[clap(short, long, default_value = "")]
    sqlite: String,
}

fn main() {
    let _opts: Opts = Opts::parse();
    // if opts.sqlite.len() > 0 {
    let _ = dump_sqlite("phodal.dev");
    // }
    println!("Hello, world!");
}

fn dump_sqlite(db_name: &str) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from("_fixtures").join("phodal.com");
    let _ = fs::create_dir(&path);

    let conn = Connection::open(db_name)?;
    // let fields = vec!["id", "keywords", "title", "slug", "description", "content", "first_name", "last_name", "email"];
    let mut query = conn.prepare("
SELECT blog_blogpost.id, blog_blogpost.keywords_string, blog_blogpost.title, blog_blogpost.slug, blog_blogpost.content,
       auth_user.first_name, auth_user.last_name, auth_user.email
FROM blog_blogpost
         INNER JOIN auth_user
                    ON blog_blogpost.user_id = auth_user.id
")?;

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
                matter.fields.insert(name.to_string(), format!("'{:}'", value));
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
