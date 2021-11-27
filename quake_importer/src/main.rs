use clap::Parser;
use std::path::PathBuf;
use rusqlite::Row;
use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::front_matter::FrontMatter;
use rusqlite::types::ValueRef;
use std::fs;

mod mezzanine;
mod apple_notes;
mod sql_to_file;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    /// custom SQLite 3 to fields
    #[clap(short, long, default_value = "")]
    sqlite: String,
}

fn main() {
    let _opts: Opts = Opts::parse();
}
