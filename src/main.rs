use std::error::Error;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use std::{env, fs};

use clap::Parser;
use futures::executor::block_on;
use tracing::{debug, error, info};

use quake_core::entry::entry_defines::EntryDefines;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::quake::QuakeActionNode;
use quake_core::QuakeConfig;
// use quake_gui::launch as launch_gui;
use quake_tui::tui_main_loop;
use static_dump::static_dump;

use crate::server::quake_rocket;
use crate::usecases::generate_usecases::generate_by_flow;

pub mod cli;
pub mod helper;
pub mod server;
pub mod static_dump;
pub mod usecases;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(version = "0.3.0", author = "Inherd <quake@inherd.org>")]
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
    /// GUI
    Gui(Gui),
    /// dump page for web deploy
    Static(StaticDump),
    /// generate content and entry from source
    Generate(Generate),
}

#[derive(Parser)]
pub struct Terminal {}

#[derive(Parser)]
pub struct StaticDump {
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,

    #[clap(short, long, default_value = "output")]
    output: String,
}

#[derive(Parser)]
pub struct WebServer {
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,
    // #[clap(short, long)]
    // /// auto watch entry change, and feed to search engine
    // watch: bool,
}

#[derive(Parser)]
pub struct Init {
    /// init by path
    #[clap(short, long, default_value = ".")]
    path: String,

    #[clap(short, long)]
    /// download web.zip from GitHub
    download: bool,
}

/// generate from target with filter
#[derive(Parser, Debug)]
pub struct Generate {
    #[clap(short, long, default_value = ".quake.yaml")]
    config: String,
    /// transflow like: `from("source path").to("target entry").filter("*.pdf")`,
    #[clap(short, long)]
    flow: String,
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

#[derive(Parser, Debug)]
pub struct Gui {
    #[clap(required = true, value_parser)]
    path: PathBuf,
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
        SubCommand::Init(init) => init_projects(init).await?,
        SubCommand::Cmd(cmd) => {
            let conf = config_quake(&cmd)?;
            if !cmd.input.is_empty() {
                let expr = QuakeActionNode::from_text(cmd.input.as_str())?;
                cli::action(expr, conf)?
            }
        }
        SubCommand::Server(server) => {
            let config = load_config(&server.config)?;
            run_server(server, config).await
        }
        SubCommand::Tui(_) => {
            tui_main_loop()?;
        }
        SubCommand::Static(dump) => {
            let config = load_config(&dump.config)?;
            static_dump(config);
        }
        SubCommand::Generate(generate) => {
            let conf = load_config(&generate.config)?;
            generate_by_flow(&generate.flow, &conf)?;
        }
        SubCommand::Gui(_gui) => {
            // launch_gui(gui.path)?;
        }
    }

    Ok(())
}

async fn run_server(_server: WebServer, _config: QuakeConfig) {
    #[allow(clippy::async_yields_async)]
    let _ = block_on(async { quake_rocket().launch() }).await;
}

fn setup_log() {
    use tracing_subscriber::prelude::*;
    let filter_layer = tracing_subscriber::filter::LevelFilter::WARN;
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

fn load_config(path: &str) -> Result<QuakeConfig, Box<dyn Error>> {
    let content =
        fs::read_to_string(path).expect("lost .quake.yaml config, please run `quake init`");
    let conf: QuakeConfig = serde_yaml::from_str(content.as_str()).expect("serde .quake.yml error");

    Ok(conf)
}

async fn init_projects(config: Init) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(&config.path)?;

    let workspace = PathBuf::from(&config.path);
    let define = workspace.join(EntryPaths::entries_define());

    let path = workspace.join(EntryPaths::quake_config());

    // todo: make default config to template
    let quake_config = QuakeConfig {
        workspace: config.path,
        editor: "".to_string(),
        search_url: "http://127.0.0.1:7700".to_string(),
        server_location: "web".to_string(),
        // debug_level: "normal".to_string(),
        auto_feed: false,
        port: 8000,
    };

    fs::write(&path, serde_yaml::to_string(&quake_config)?)?;
    debug!(
        "create {:} in {:?}",
        EntryPaths::quake_config(),
        &path.display()
    );

    let todo_define = "
- type: todo
  display: Todo
  properties:
    - title: Title
    - author: String
";

    let file = EntryDefines {
        entries: serde_yaml::from_str(todo_define).unwrap(),
    };

    fs::write(&define, serde_yaml::to_string(&file)?)?;
    debug!("create default entry defines in {:?}", &define.display());

    if config.download {
        let target = format!(
            "https://github.com/phodal/quake/releases/download/v{}/web.zip",
            VERSION
        );
        debug!("download web.zip from {}", target);

        let response = reqwest::get(&target).await?.bytes().await?;
        unzip_all(Cursor::new(response.to_vec()), &workspace)?;
    }

    Ok(())
}

fn unzip_all(reader: Cursor<Vec<u8>>, workspace: &Path) -> Result<(), Box<dyn Error>> {
    let mut archive = zip::ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = match file.enclosed_name() {
            Some(path) => workspace.join(path),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                debug!("Plugin {} comment: {}", i, comment);
            }
        }

        if (file.name()).ends_with('/') {
            info!("File {} extracted to \"{}\"", i, out_path.display());
            fs::create_dir_all(&out_path)?;
        } else {
            info!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                out_path.display(),
                file.size()
            );
            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))?;
            }
        }
    }
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

    use crate::{init_projects, process_cmd, Command, Init, Opts, SubCommand};

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

            let error_msg = "QuakeError(\"unknown entry action: QuakeActionNode { entry: \\\"story\\\", action: \\\"dddd\\\", text: \\\"\\\", parameters: [] }\")";
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
                    download: false,
                }),
            })
            .await
            .unwrap();

            process_cmd(Opts {
                cmd: SubCommand::Cmd(command),
            })
            .await
            .unwrap();

            let paths =
                EntryPaths::init(&format!("{:}", PathBuf::from(test_dir).display()), "water");

            let content = fs::read_to_string(paths.entry_path.join("0001-samples.md")).unwrap();
            let file = EntryFile::from(content.as_str(), 1).unwrap();

            let title = file.property("title");
            assert_eq!(title.unwrap(), "samples");

            fs::remove_dir_all(test_dir).unwrap();
        });
    }

    #[test]
    #[ignore = "need tokio runtime"]
    fn should_download_webapp_dist() {
        task::block_on(async {
            let test_dir = "test_dir";
            process_cmd(Opts {
                cmd: SubCommand::Init(Init {
                    path: test_dir.to_string(),
                    download: true,
                }),
            })
            .await
            .unwrap();
        })
    }

    fn config_dir() -> PathBuf {
        PathBuf::from("_fixtures")
            .join("configs")
            .join(".quake.yaml")
    }

    #[ignore]
    #[test]
    fn placeholder() {
        let paths = EntryPaths::init("examples", "notes");
        sync_in_path(&paths).unwrap();

        let paths = EntryPaths::init("examples", "blog");
        sync_in_path(&paths).unwrap();
    }

    #[ignore]
    #[test]
    fn sync_todo() {
        let paths = EntryPaths::init("examples", "microsoft_todo");
        sync_in_path(&paths).unwrap();
    }

    #[ignore]
    #[test]
    fn test_init() {
        task::block_on(async {
            init_projects(Init {
                path: "sample".to_string(),
                download: true,
            })
            .await
            .unwrap()
        })
    }
}
