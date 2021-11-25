extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};

use action::entry_action;
use quake_core::input_parser::InputParser;
use quake_core::quake_config::QuakeConfig;

pub mod entry;
pub mod action;
pub mod helper;

fn config_file(config: &str, editor: &str) -> QuakeConfig {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(config)).unwrap();
    let mut conf: QuakeConfig = settings.try_into().unwrap();

    if !editor.is_empty() {
        conf.editor = editor.to_string();
    }

    conf
}

fn main() {
    let matches = App::new("quake")
        .about("Another simple opensource knowledge management tool for geek.")
        .version("0.1.0")
        .author("Phodal HUANG")
        .arg(Arg::new("editor").short('e').long("editor").value_name("").takes_value(true))
        .arg(Arg::new("input").short('i').long("input").value_name("").takes_value(true))
        .arg(Arg::new("config").short('c').long("config").value_name(".quake.yml").takes_value(true))
        .subcommand(
            App::new("init")
                .about("init quake projects"),
        )
        .get_matches();

    let mut input = "";
    if let Some(o) = matches.value_of("input") {
        input = o;
    }

    let mut config = ".quake.yaml";
    if let Some(o) = matches.value_of("config") {
        config = o;
    }

    let mut editor = ".quake.yaml";
    if let Some(o) = matches.value_of("editor") {
        editor = o;
    }

    let conf: QuakeConfig = config_file(config, editor);

    if input.len() > 0 {
        let expr = InputParser::from(input);
        match expr.object.to_lowercase().as_str() {
            "todo" => {
                if let Err(err) = entry_action::create_action(expr, conf) {
                    println!("{:?}", err)
                }
            }
            _ => {
                if let Err(err) = entry_action::create_action(expr, conf) {
                    println!("{:?}", err)
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {}
}