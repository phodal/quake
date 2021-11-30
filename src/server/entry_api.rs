use std::fs;
use std::path::PathBuf;

use rocket::response::content;
use rocket::response::content::Json;
use rocket::response::status::NotFound;
use rocket::State;
use rocket::tokio::task::spawn_blocking;
use quake_core::entry::entry_file::EntryFile;

use crate::action::file_process;
use crate::server::{ApiError, QuakeServerConfig};

#[get("/<entry_type>", rank = 3)]
pub(crate) async fn get_entries(entry_type: &str) -> Json<String> {
    let request_url = format!("http://127.0.0.1:7700/indexes/{:}/documents", entry_type);

    let vec = spawn_blocking(|| content::Json(
        reqwest::blocking::get(request_url)
            .unwrap()
            .text()
            .unwrap(),
    )).await.map_err(|e| ApiError {
        msg: format!("{:?}", e)
    }).unwrap();

    vec
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct EntryResponse {
    content: String
}

#[get("/<entry_type>/<id>", rank = 3)]
pub(crate) async fn get_entry(entry_type: &str, id: usize, config: &State<QuakeServerConfig>) -> Result<Json<String>, NotFound<Json<String>>> {
    let base_path = PathBuf::from(&config.workspace).join(entry_type);
    let prefix = file_process::file_prefix(id);
    let vec = file_process::filter_by_prefix(base_path, prefix);
    if vec.len() == 0 {
        return Err(NotFound(content::Json(serde_json::to_string(&ApiError {
            msg: "file not found".to_string()
        }).unwrap())));
    }
    let file_path = vec[0].clone();

    let str = fs::read_to_string(file_path).expect("cannot read entries-define.yaml");
    let file = EntryFile::from(str.as_str()).unwrap();
    let string = serde_json::to_string(&file).unwrap();

    return Ok(content::Json(string));
}
