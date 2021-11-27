extern crate config;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use clap::Parser;
use action::entry_action;

use quake_core::input_parser::InputParser;
use quake_core::quake_config::QuakeConfig;

pub mod action;
pub mod helper;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    /// config path
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,
    /// like `todo.add: hello world`
    #[clap(short, long)]
    input: String,
    /// config the editor
    #[clap(short, long, default_value = "")]
    editor: String,
}

fn config(opts: &Opts) -> QuakeConfig {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(&opts.config)).unwrap();
    let mut conf: QuakeConfig = settings.try_into().unwrap();

    if !opts.editor.is_empty() {
        conf.editor = opts.editor.clone();
    }

    conf
}

fn main() {
    let opts: Opts = Opts::parse();

    let conf: QuakeConfig = config(&opts);

    if opts.input.len() > 0 {
        let expr = InputParser::from(opts.input.as_str());
        if let Err(err) = entry_action::create_action(expr, conf) {
            println!("{:?}", err)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::entry_action::{EntryPaths, sync_in_path};

    #[ignore]
    #[test]
    fn placeholder() {
        let paths = EntryPaths::init(&"_fixtures".to_string(), &"notes".to_string());
        sync_in_path(&paths).unwrap();

        let paths = EntryPaths::init(&"_fixtures".to_string(), &"phodal.com".to_string());
        sync_in_path(&paths).unwrap();
    }
}