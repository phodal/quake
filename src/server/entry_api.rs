use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use rocket::response::status::NotFound;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::State;
use rocket::tokio::task::spawn_blocking;

use quake_core::entry::entry_file::EntryFile;

use crate::action::entry_usecases;
use crate::helper::file_process;
use crate::server::{ApiError, QuakeServerConfig};

#[get("/<entry_type>", rank = 3)]
pub(crate) async fn get_entries(entry_type: &str) -> Json<String> {
    let request_url = format!("http://127.0.0.1:7700/indexes/{:}/documents", entry_type);

    let vec = spawn_blocking(||         reqwest::blocking::get(request_url)
        .unwrap()
        .text()
        .unwrap()).await.map_err(|e| ApiError {
        msg: format!("{:?}", e)
    }).unwrap();

    Json(vec)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct EntryResponse {
    content: String,
}

#[post("/<entry_type>/new?<text>")]
pub(crate) async fn create_entry(entry_type: String, text: String, config: &State<QuakeServerConfig>) -> Result<Json<EntryFile>, NotFound<Json<ApiError>>> {
    let workspace = config.workspace.to_string();
    match entry_usecases::create_entry(&workspace, &entry_type, &text) {
        Ok((_path, file)) => {
            return Ok(Json(file));
        }
        Err(err) => {
            return Err(NotFound(Json(ApiError {
                msg: err.to_string()
            })));
        }
    }
}

#[get("/<entry_type>/<id>", rank = 3)]
pub(crate) async fn get_entry(entry_type: &str, id: usize, config: &State<QuakeServerConfig>) -> Result<Json<EntryFile>, NotFound<Json<ApiError>>> {
    let base_path = PathBuf::from(&config.workspace).join(entry_type);
    let prefix = file_process::file_prefix(id);
    let vec = file_process::filter_by_prefix(base_path, prefix);
    if vec.len() == 0 {
        return Err(NotFound(Json(ApiError {
            msg: "file not found".to_string()
        })));
    }
    let file_path = vec[0].clone();

    let str = fs::read_to_string(file_path).expect("cannot read entry type");
    let file = EntryFile::from(str.as_str()).unwrap();

    return Ok(Json(file));
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EntryUpdate {
    fields: HashMap<String, usize>,
}

#[post("/<entry_type>/<id>", data="<entry>")]
pub(crate) async fn update_entry(entry_type: &str, id: usize, entry: Json<EntryUpdate>,config: &State<QuakeServerConfig>) {
    println!("{:?}", entry);
    println!("{:?}", config);
}