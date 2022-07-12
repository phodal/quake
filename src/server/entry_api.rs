use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::{get, post};

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::{entry_by_path, EntryDefines};
use quake_core::helper::file_filter;
use quake_core::usecases::entry_usecases;
use quake_core::QuakeConfig;

use crate::helper::exec_wrapper::meili_exec::feed_document_async;
use crate::server::ApiError;

#[get("/defines")]
pub(crate) async fn get_entry_defines(conf: &State<QuakeConfig>) -> String {
    let path = PathBuf::from(&conf.workspace);
    let defines = EntryDefines::from_path(&path.join(EntryPaths::entries_define()));
    serde_json::to_string(&defines).unwrap()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct EntryResponse {
    content: String,
}

#[post("/<entry_type>?<text>")]
pub(crate) async fn create_entry(
    entry_type: &str,
    text: &str,
    config: &State<QuakeConfig>,
) -> Result<Json<EntryFile>, NotFound<Json<ApiError>>> {
    let workspace = config.workspace.to_string();
    return match entry_usecases::create_entry(&workspace, entry_type, text) {
        Ok((path, file)) => {
            feed_entry(&config.search_url, entry_type, &path, &config.workspace);
            Ok(Json(file))
        }
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
    let file_path = entry_by_id(entry_type, id, config)?;

    let str = fs::read_to_string(file_path).expect("cannot read entry type");
    let file = EntryFile::from(str.as_str(), id).unwrap();

    Ok(Json(file))
}

pub fn entry_by_id(
    entry_type: &str,
    id: usize,
    config: &State<QuakeConfig>,
) -> Result<PathBuf, NotFound<Json<ApiError>>> {
    let base_path = PathBuf::from(&config.workspace).join(entry_type);
    let entries = file_filter::filter_by_prefix(base_path, EntryFile::file_prefix(id));
    if entries.is_empty() {
        return Err(NotFound(Json(ApiError {
            msg: "file not found".to_string(),
        })));
    }

    Ok(entries[0].clone())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EntryUpdate {
    properties: HashMap<String, String>,
}

#[post("/<entry_type>/<id>", data = "<entry>")]
pub(crate) async fn update_entry(
    entry_type: &str,
    id: usize,
    entry: Json<EntryUpdate>,
    config: &State<QuakeConfig>,
) -> Result<Json<EntryFile>, NotFound<Json<ApiError>>> {
    let path = PathBuf::from(&config.workspace).join(entry_type);

    return match entry_usecases::update_entry_properties(path, entry_type, id, &entry.properties) {
        Ok((path, file)) => {
            feed_entry(&config.search_url, entry_type, &path, &config.workspace);
            Ok(Json(file))
        }
        Err(err) => Err(NotFound(Json(ApiError {
            msg: err.to_string(),
        }))),
    };
}

pub fn feed_entry(server_url: &str, index_name: &str, path: &Path, workspace: &str) {
    if let Ok((_typ, file)) = entry_by_path::entry_file_dump(path, &PathBuf::from(workspace)) {
        let _ = feed_document_async(server_url, index_name, &file);
    };
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fs;
    use std::io::Read;

    use rocket::form::validate::Contains;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use quake_core::entry::entry_paths::EntryPaths;

    use crate::quake_rocket;
    use crate::server::entry_api::EntryUpdate;

    #[test]
    fn crud_for_entry() {
        // create entry
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let response = client.post("/entry/test_quake?text=demo").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // update entry date
        let created_time = "2021-12-13 20:45:51";
        let req = create_update_req(created_time);
        let string = serde_json::to_string(&req).unwrap();
        let response = client.post("/entry/test_quake/1").body(string).dispatch();
        assert_eq!(response.status(), Status::Ok);

        // assert for entry time is update
        let mut response = client.get("/entry/test_quake/1").dispatch();
        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        assert_eq!(response.status(), Status::Ok);
        assert!(res.contains(created_time));
        assert!(res.contains("\"id\":1"));
        assert!(res.contains("\"title\":\"demo\""));

        let paths = EntryPaths::init("examples", "test_quake");
        fs::remove_dir_all(paths.entry_path).unwrap();
    }

    fn create_update_req(time: &str) -> EntryUpdate {
        let mut fields: HashMap<String, String> = HashMap::new();
        fields.insert("created_date".to_string(), time.to_string());
        fields.insert("updated_date".to_string(), time.to_string());

        EntryUpdate { properties: fields }
    }
}
