use std::error::Error;
use std::fs;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use futures::{
    channel::mpsc::{channel, Receiver},
    future, SinkExt, StreamExt,
};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};

use quake_core::entry::entry_defines::EntryDefines;
use quake_core::entry::entry_file::EntryFile;
use quake_core::parser::action_parser::ActionDefine;
use quake_core::QuakeConfig;
use quake_tui::tui_main_loop;

use crate::helper::file_process::type_from_md_path;
use crate::helper::meili_exec::feed_entry;
use crate::server::quake_rocket;

pub mod action;
pub mod cli;
pub mod helper;
pub mod server;
pub mod tui;

#[derive(Parser)]
#[clap(version = "0.0.1", author = "Phodal HUANG<h@phodal.com>")]
pub struct Opts {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    /// init project
    Init(Init),
    /// command for CRUD entries
    Cmd(Command),
    /// web server for run
    Server(WebServer),
    /// Terminal UI
    Tui(Terminal),
}

#[derive(Parser)]
pub struct Terminal {}

#[derive(Parser)]
pub struct WebServer {
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,
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

#[rocket::main]
async fn main() {
    let mut stdout = stdout();
    let opts: Opts = Opts::parse();
    if let Err(err) = process_cmd(opts).await {
        stdout.write(format!("{:?}", err).as_bytes()).unwrap();
    }
}

pub async fn process_cmd(opts: Opts) -> Result<(), Box<dyn Error>> {
    match opts.cmd {
        SubCommand::Init(init) => init_projects(init)?,
        SubCommand::Cmd(cmd) => {
            let conf = config_quake(&cmd)?;
            if cmd.input.len() > 0 {
                let expr = ActionDefine::from(cmd.input.as_str())?;
                cli::action(expr, conf)?
            }
        }
        SubCommand::Server(server) => {
            let path = load_config(&server.config)?.workspace;
            futures::executor::block_on(async {
                let (_s, _g) = future::join(quake_rocket().launch(), async_watch(path)).await;
            });
        }
        SubCommand::Tui(_) => {
            tui_main_loop()?;
        }
    }

    Ok(())
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

    fs::write(path, serde_yaml::to_string(&config)?)?;

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

    fs::write(define, serde_yaml::to_string(&file)?)?;

    Ok(())
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    })?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    println!("start watch: {:?}", path.as_ref());
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                println!("feed_by_event {:?}", event);
                match feed_by_event(event) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("watch error: {:?}", err)
                    }
                };
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn feed_by_event(event: Event) -> Result<(), Box<dyn Error>> {
    for path in event.paths {
        if path.is_dir() {
            continue;
        }

        if let Some(ext) = path.extension() {
            if !ext.eq("md") {
                continue;
            }
        }

        let (typ, file) = entry_file_by_path(&path);
        let string = serde_json::to_string(&file)?;
        feed_entry(&typ, &string)?;
    }

    Ok(())
}

pub fn entry_file_by_path(path: &PathBuf) -> (String, EntryFile) {
    let typ = type_from_md_path(&path).unwrap();
    let file_name = path.file_name().unwrap();
    let id = EntryFile::id_from_name(format!("{:}", file_name.to_str().unwrap()).as_str()).unwrap();
    let content = fs::read_to_string(&path).unwrap();
    let file = EntryFile::from(content.as_str(), id).unwrap();
    (typ, file)
}

#[cfg(test)]
mod tests {
    use async_std::task;
    use std::fs;
    use std::path::PathBuf;

    use quake_core::entry::entry_file::EntryFile;

    use crate::action::entry_paths::EntryPaths;
    use crate::action::entry_usecases::sync_in_path;
    use crate::{entry_file_by_path, process_cmd, Command, Init, Opts, SubCommand};

    #[test]
    fn entry_by_path() {
        let buf = PathBuf::from("_fixtures")
            .join("todo")
            .join("0001-time-support.md");

        let (typ, file) = entry_file_by_path(&buf);
        assert_eq!(typ, "todo".to_string());
        assert_eq!(1, file.id);
    }

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

            let error_msg = "QuakeError(\"unknown entry action: ActionDefine { object: \\\"story\\\", action: \\\"dddd\\\", text: \\\"\\\", parameters: [] }\")";
            assert_eq!(format!("{:?}", expected), error_msg);
        });
    }

    #[test]
    fn should_create_test_entry() {
        task::block_on(async {
            let test_dir = "test_dir";

            let conf_dir = PathBuf::from("_fixtures")
                .join("configs")
                .join(".quake.yaml");

            let command = Command {
                config: format!("{:}", conf_dir.display()),
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

            let test_path = PathBuf::from("test_dir");
            let paths =
                EntryPaths::init(&format!("{:}", test_path.display()), &"water".to_string());

            let content = fs::read_to_string(paths.base.join("0001-samples.md")).unwrap();
            let file = EntryFile::from(content.as_str(), 1).unwrap();

            let title = file.field("title");
            assert_eq!(title.unwrap(), "samples");

            fs::remove_dir_all(test_dir).unwrap();
        });
    }

    #[ignore]
    #[test]
    fn placeholder() {
        let paths = EntryPaths::init(&"_fixtures".to_string(), &"notes".to_string());
        sync_in_path(&paths).unwrap();

        let paths = EntryPaths::init(&"_fixtures".to_string(), &"blog".to_string());
        sync_in_path(&paths).unwrap();
    }

    #[ignore]
    #[test]
    fn sync_todo() {
        let paths = EntryPaths::init(&"_fixtures".to_string(), &"microsoft_todo".to_string());
        sync_in_path(&paths).unwrap();
    }
}
