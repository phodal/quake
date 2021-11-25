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
    let _ = dump_sqlite();
    // }
    println!("Hello, world!");
}

fn dump_sqlite() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("phodal.dev")?;

    let mut query = conn.prepare("select id, keywords_string, title, slug, description, content from blog_blogpost")?;
    let mut rows = query.query([])?;
    while let Some(row) = rows.next()? {
        println!("{:?}", row.get_ref(0))
    };

    Ok(())
}
