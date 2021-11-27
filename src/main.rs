extern crate config;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use clap::Parser;
use action::entry_action;

use quake_core::input_parser::InputParser;
use quake_core::quake_config::QuakeConfig;

pub mod action;
pub mod helper;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
struct Opts {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    /// init project
    Init(Init),
    /// command for CRUD entries
    Command(Command),
    /// web server for run
    Server(WebServer),
    /// Terminal UI
    Tui(Terminal),
}

#[derive(Parser)]
struct Terminal {}

#[derive(Parser)]
struct Init {
    /// init by path
    #[clap(short, long, default_value = ".")]
    path: String,
}

#[derive(Parser)]
struct Command {
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

#[derive(Parser)]
struct WebServer {
    /// Print debug info
    #[clap(short)]
    debug: bool,
    /// init by path
    #[clap(short, long, default_value = ".")]
    path: String,
}

fn config(cmd: &Command) -> QuakeConfig {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(&cmd.config)).unwrap();
    let mut conf: QuakeConfig = settings.try_into().unwrap();

    if !cmd.editor.is_empty() {
        conf.editor = cmd.editor.clone();
    }

    conf
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Init(init) => {
            if let Err(err) = init_projects(init) {
                println!("{:?}", err)
            }
        }
        SubCommand::Command(cmd) => {
            let conf: QuakeConfig = config(&cmd);

            if cmd.input.len() > 0 {
                let expr = InputParser::from(cmd.input.as_str());
                if let Err(err) = entry_action::create_action(expr, conf) {
                    println!("{:?}", err)
                }
            }
        }
        SubCommand::Server(_) => {}
        SubCommand::Tui(_) => {}
    }
}

fn init_projects(config: Init) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(&config.path).join(".quake.yaml");
    let config = QuakeConfig {
        path: config.path,
        editor: "vim".to_string(),
    };

    fs::write(path, serde_yaml::to_string(&config)?)?;
    Ok(())
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