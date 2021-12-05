use std::fs;
use std::path::PathBuf;

use rocket::get;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
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
