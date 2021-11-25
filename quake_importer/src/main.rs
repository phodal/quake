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
    let opts: Opts = Opts::parse();
    // if opts.sqlite.len() > 0 {
    let _ = dump_sqlite();
    // }
    println!("Hello, world!");
}

fn dump_sqlite() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("phodal.dev")?;

    let result = conn.query_row("select id from blog_blogpost", [], |row| {
        row.get::<_, i64>(0)
    });
    println!("{:?}", result);
    Ok(())
}
