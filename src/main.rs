use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use futures::future;
use helper::entry_watcher;
use tracing::{debug, error};

use quake_core::entry::entry_defines::EntryDefines;
use quake_core::quake::QuakeActionNode;
use quake_core::QuakeConfig;
use quake_tui::tui_main_loop;

use crate::server::quake_rocket;

pub mod cli;
pub mod helper;
pub mod server;

#[derive(Parser)]
#[clap(version = "0.1.2", author = "Inherd <quake@inherd.org>")]
pub struct Opts {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    /// init quake workspace
    Init(Init),
    /// cli command for CRUD entries
    Cmd(Command),
    /// web server for run
    Server(WebServer),
    /// terminal UI
    Tui(Terminal),
}

#[derive(Parser)]
pub struct Terminal {}

#[derive(Parser)]
pub struct WebServer {
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,

    #[clap(short, long)]
    /// auto watch entry change, and feed to search engine
    watch: bool,
}

#[derive(Parser)]
pub struct Init {
    /// init by path
    #[clap(short, long, default_value = ".")]
    path: String,
}

#[derive(Parser, Debug)]
pub struct Command {
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

#[rocket::main]
async fn main() {
    let opts: Opts = Opts::parse();

    setup_log();
    if let Err(err) = process_cmd(opts).await {
        error!("{:?}", err);
    }
}

pub async fn process_cmd(opts: Opts) -> Result<(), Box<dyn Error>> {
    match opts.cmd {
        SubCommand::Init(init) => init_projects(init)?,
        SubCommand::Cmd(cmd) => {
            let conf = config_quake(&cmd)?;
            if cmd.input.len() > 0 {
                let expr = QuakeActionNode::action_from_text(cmd.input.as_str())?;
                cli::action(expr, conf)?
            }
        }
        SubCommand::Server(server) => {
            let config = load_config(&server.config)?;
            let path = config.workspace;
            let search_url = config.search_url;

            if server.watch {
                futures::executor::block_on(async {
                    let (_s, _g) = future::join(
                        quake_rocket().launch(),
                        entry_watcher::async_watch(path, search_url),
                    )
                    .await;
                });
            } else {
                let _ = futures::executor::block_on(async { quake_rocket().launch() }).await;
            }
        }
        SubCommand::Tui(_) => {
            tui_main_loop()?;
        }
    }

    Ok(())
}

fn setup_log() {
    use tracing_subscriber::prelude::*;
    let filter_layer = tracing_subscriber::filter::LevelFilter::DEBUG;
    let fmt_layer = tracing_subscriber::fmt::layer().with_target(true);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

fn config_quake(cmd: &Command) -> Result<QuakeConfig, Box<dyn Error>> {
    let mut conf = load_config(&cmd.config)?;

    if !cmd.editor.is_empty() {
        conf.editor = cmd.editor.clone();
    }

    Ok(conf)
}

fn load_config(path: &String) -> Result<QuakeConfig, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let conf: QuakeConfig = serde_yaml::from_str(content.as_str())?;

    Ok(conf)
}

fn init_projects(config: Init) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(&config.path)?;

    let path = PathBuf::from(&config.path).join(".quake.yaml");
    let define = PathBuf::from(&config.path).join("entries-define.yaml");

    let config = QuakeConfig {
        workspace: config.path.clone(),
        editor: "".to_string(),
        search_url: "http://127.0.0.1:7700".to_string(),
        server_location: "web".to_string(),
    };

    fs::write(&path, serde_yaml::to_string(&config)?)?;
    debug!("create .quake.yaml in {:?}", &path.display());

    let todo_define = "
- type: todo
  display: Todo
  fields:
    - title: Title
    - author: String
";

    let file = EntryDefines {
        entries: serde_yaml::from_str(todo_define).unwrap(),
    };

    fs::write(&define, serde_yaml::to_string(&file)?)?;
    debug!("create default entry defines in {:?}", &define.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use async_std::task;

    use quake_core::entry::entry_file::EntryFile;
    use quake_core::entry::entry_paths::EntryPaths;
    use quake_core::usecases::entry_usecases::sync_in_path;

    use crate::{process_cmd, Command, Init, Opts, SubCommand};

    #[test]
    fn should_throw_not_exist_cmds() {
        task::block_on(async {
            let command = Command {
                config: ".quake.yaml".to_string(),
                input: "story.dddd".to_string(),
                editor: "".to_string(),
            };

            let expected = process_cmd(Opts {
                cmd: SubCommand::Cmd(command),
            })
            .await
            .expect_err("");

            let error_msg = "QuakeError(\"unknown entry action: QuakeActionNode { object: \\\"story\\\", action: \\\"dddd\\\", text: \\\"\\\", parameters: [] }\")";
            assert_eq!(format!("{:?}", expected), error_msg);
        });
    }

    #[test]
    fn should_create_test_entry() {
        task::block_on(async {
            let test_dir = "test_dir";

            let command = Command {
                config: format!("{:}", config_dir().display()),
                input: "water.add: samples".to_string(),
                editor: "".to_string(),
            };

            process_cmd(Opts {
                cmd: SubCommand::Init(Init {
                    path: test_dir.to_string(),
                }),
            })
            .await
            .unwrap();

            process_cmd(Opts {
                cmd: SubCommand::Cmd(command),
            })
            .await
            .unwrap();

            let paths = EntryPaths::init(
                &format!("{:}", PathBuf::from(test_dir).display()),
                &"water".to_string(),
            );

            let content = fs::read_to_string(paths.base.join("0001-samples.md")).unwrap();
            let file = EntryFile::from(content.as_str(), 1).unwrap();

            let title = file.field("title");
            assert_eq!(title.unwrap(), "samples");

            fs::remove_dir_all(test_dir).unwrap();
        });
    }

    fn config_dir() -> PathBuf {
        let conf_dir = PathBuf::from("_fixtures")
            .join("configs")
            .join(".quake.yaml");
        conf_dir
    }

    #[ignore]
    #[test]
    fn placeholder() {
        let paths = EntryPaths::init(&"examples".to_string(), &"notes".to_string());
        sync_in_path(&paths).unwrap();

        let paths = EntryPaths::init(&"examples".to_string(), &"blog".to_string());
        sync_in_path(&paths).unwrap();
    }

    #[ignore]
    #[test]
    fn sync_todo() {
        let paths = EntryPaths::init(&"examples".to_string(), &"microsoft_todo".to_string());
        sync_in_path(&paths).unwrap();
    }
}
