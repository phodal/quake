extern crate config;

use clap::Parser;

use quake_core::input_parser::InputParser;
use quake_core::quake_config::QuakeConfig;

pub mod entry;
pub mod action;
pub mod helper;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,
    #[clap(short, long)]
    input: String,
    #[clap(short, long, default_value = "")]
    editor: String,
}

fn config(file: &String) -> QuakeConfig {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(file)).unwrap();

    settings.try_into().unwrap()
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut conf: QuakeConfig = config(&opts.config);
    if !opts.editor.is_empty() {
        conf.editor = opts.editor;
    }

    if opts.input.len() > 0 {
        let expr = InputParser::from(opts.input.as_str());
        match expr.object.to_lowercase().as_str() {
            "todo" => {
                action::create_action(expr, conf);
            }
            _ => {
                action::create_action(expr, conf);
            }
        }
    }
}
