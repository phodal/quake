use rocket::fs::NamedFile;
use std::fs;
use std::path::PathBuf;

use quake_core::entry::EntryDefines;
use quake_core::quake::QuakeTransflowNode;
use rocket::get;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::tokio::task::spawn_blocking;
use rocket::State;

use quake_core::transflow::Transflow;
use quake_core::QuakeConfig;

use crate::action::entry_paths::EntryPaths;
use crate::server::ApiError;

#[get("/define")]
pub(crate) async fn transflow_defines(
    config: &State<QuakeConfig>,
) -> Result<Json<Vec<Transflow>>, NotFound<Json<ApiError>>> {
    let path = PathBuf::from(config.workspace.clone());
    let yaml = path.join(EntryPaths::transflows_yaml());

    let content = match fs::read_to_string(yaml) {
        Ok(content) => content,
        Err(err) => {
            return Err(NotFound(Json(ApiError {
                msg: err.to_string(),
            })));
        }
    };

    let flows: Vec<Transflow> = match serde_yaml::from_str(&*content) {
        Ok(content) => content,
        Err(err) => {
            return Err(NotFound(Json(ApiError {
                msg: err.to_string(),
            })));
        }
    };

    Ok(Json(flows))
}

#[get("/script")]
pub(crate) async fn transfunc_script(config: &State<QuakeConfig>) -> Option<NamedFile> {
    let path = PathBuf::from(config.workspace.clone());
    let fs = path.join(EntryPaths::transfuncs());

    let file = NamedFile::open(fs);
    file.await.ok()
}

#[get("/query?<input>")]
pub(crate) async fn translate(
    input: String,
    config: &State<QuakeConfig>,
) -> Result<Json<Transflow>, Json<ApiError>> {
    let path = PathBuf::from(&config.workspace).join("entries-define.yaml");

    let result = spawn_blocking(|| {
        let entries_str = fs::read_to_string(path).expect("cannot read entries-define.yaml");
        let entries: EntryDefines = serde_yaml::from_str(&*entries_str).unwrap();
        entries.entries
    })
    .await;

    let defines = match result {
        Ok(defines) => defines,
        Err(err) => {
            return Err(Json(ApiError {
                msg: format!("{:?}", err),
            }))
        }
    };

    let node = QuakeTransflowNode::from_text(input.as_str()).unwrap();

    let flow = Transflow::from(defines, node);

    Ok(Json(flow))
}
