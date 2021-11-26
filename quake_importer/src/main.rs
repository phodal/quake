use clap::Parser;

mod mezzanine;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    /// custom SQLite 3 to fields
    #[clap(short, long, default_value = "")]
    sqlite: String,
}

fn main() {
    let _opts: Opts = Opts::parse();
    // dump_phodal_com();
}
