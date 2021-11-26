use std::error::Error;
use clap::Parser;
use rusqlite::Connection;

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
    let conn = Connection::open(db_name)?;

    let mut query = conn.prepare("
SELECT blog_blogpost.id, blog_blogpost.keywords_string, blog_blogpost.title, blog_blogpost.slug, blog_blogpost.description, blog_blogpost.content,
       auth_user.first_name, auth_user.last_name, auth_user.email
FROM blog_blogpost
         INNER JOIN auth_user
                    ON blog_blogpost.user_id = auth_user.id
")?;
    let mut rows = query.query([])?;
    while let Some(row) = rows.next()? {
        // for column in row.column_names() {
        //
        // }
    };

    Ok(())
}
