use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::task::spawn_blocking;
use rocket::State;
use rocket::{get, post};

use quake_core::entry::entry_file::EntryFile;
use quake_core::entry::entry_paths::EntryPaths;
use quake_core::helper::file_filter;
use quake_core::usecases::entry_usecases;
use quake_core::QuakeConfig;

use crate::server::helper::csv_to_json::csv_to_json;
use crate::server::ApiError;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct EntryResponse {
    content: String,
}

#[post("/<entry_type>?<text>")]
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

        let paths = EntryPaths::init(&"examples".to_string(), &"test_quake".to_string());
        fs::remove_dir_all(paths.base).unwrap();
    }

    fn create_update_req(time: &str) -> EntryUpdate {
        let mut fields: HashMap<String, String> = HashMap::new();
        fields.insert("created_date".to_string(), time.to_string());
        fields.insert("updated_date".to_string(), time.to_string());

        let update = EntryUpdate { fields };
        update
    }
}
