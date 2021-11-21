extern crate config;

use clap::Parser;

use quake_core::concept_parser::ConceptExpr;
use quake_core::quake_config::QuakeConfig;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap(short, long)]
    input: String,
}

fn config(file: &String) -> QuakeConfig {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(file)).unwrap();

    settings.try_into().unwrap()
}

fn main() {
    let opts: Opts = Opts::parse();

    let conf = config(&opts.config);
    println!("{:?}", conf);
    if opts.input.len() > 0 {
        let expr = ConceptExpr::from(opts.input.as_str());
        println!("{:?}", expr);
    }
}