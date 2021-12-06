use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::task::spawn_blocking;
use rocket::State;
use rocket::{get, post};

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::usecases::entry_usecases;
use quake_core::usecases::file_filter;
use quake_core::QuakeConfig;

use crate::helper::csv_to_json::csv_to_json;
use crate::server::ApiError;

#[get("/<entry_type>")]
pub(crate) async fn get_entries(entry_type: &str, config: &State<QuakeConfig>) -> Redirect {
    let request_url = format!("{:}/indexes/{:}/search", &config.search_url, entry_type);
    Redirect::to(request_url)
}

#[get("/<entry_type>/from_csv")]
pub(crate) async fn get_entries_from_csv(
    entry_type: String,
    config: &State<QuakeConfig>,
) -> Result<Json<String>, NotFound<Json<ApiError>>> {
    let path = PathBuf::from(config.workspace.clone())
        .join(entry_type)
        .join(EntryPaths::entries_csv());
    let content = spawn_blocking(|| {
        let mut rdr = csv::Reader::from_reader(File::open(path).unwrap());
        csv_to_json(&mut rdr).unwrap().to_string()
    })
    .await
    .map_err(|err| ApiError {
        msg: err.to_string(),
    })
    .unwrap();

    Ok(Json(content))
}

#[get("/<entry_type>/csv")]
pub(crate) async fn get_entries_csv(
    entry_type: &str,
    config: &State<QuakeConfig>,
) -> Option<NamedFile> {
    let paths = EntryPaths::init(&config.workspace, &entry_type.to_string());
    let file = NamedFile::open(paths.entries_csv);
    file.await.ok()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct EntryResponse {
    content: String,
}

#[post("/<entry_type>/new?<text>")]
pub(crate) async fn create_entry(
    entry_type: String,
    text: String,
    config: &State<QuakeConfig>,
) -> Result<Json<EntryFile>, NotFound<Json<ApiError>>> {
    let workspace = config.workspace.to_string();
    return match entry_usecases::create_entry(&workspace, &entry_type, &text) {
        Ok((_path, file)) => Ok(Json(file)),
        Err(err) => Err(NotFound(Json(ApiError {
            msg: err.to_string(),
        }))),
    };
}

#[get("/<entry_type>/<id>")]
pub(crate) async fn get_entry(
    entry_type: &str,
    id: usize,
    config: &State<QuakeConfig>,
) -> Result<Json<EntryFile>, NotFound<Json<ApiError>>> {
    let base_path = PathBuf::from(&config.workspace).join(entry_type);
    let index = id;
    let prefix = EntryFile::file_prefix(index);
    let vec = file_filter::filter_by_prefix(base_path, prefix);
    if vec.len() == 0 {
        return Err(NotFound(Json(ApiError {
            msg: "file not found".to_string(),
        })));
    }
    let file_path = vec[0].clone();

    let str = fs::read_to_string(file_path).expect("cannot read entry type");
    let file = EntryFile::from(str.as_str(), id).unwrap();

    return Ok(Json(file));
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EntryUpdate {
    fields: HashMap<String, String>,
}

#[post("/<entry_type>/<id>", data = "<entry>")]
pub(crate) async fn update_entry(
    entry_type: &str,
    id: usize,
    entry: Json<EntryUpdate>,
    config: &State<QuakeConfig>,
) -> Result<Json<EntryFile>, NotFound<Json<ApiError>>> {
    let path = PathBuf::from(&config.workspace).join(entry_type);
    return match entry_usecases::update_entry_fields(path, entry_type, id, &entry.fields) {
        Ok(file) => Ok(Json(file)),
        Err(err) => Err(NotFound(Json(ApiError {
            msg: err.to_string(),
        }))),
    };
}
