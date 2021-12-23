use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::{get, State};

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::QuakeConfig;

use crate::server::ApiError;
use crate::usecases::reference_usecases::EntryReference;

#[get("/<entry_type>")]
pub(crate) async fn reference_by_type(
    entry_type: String,
    conf: &State<QuakeConfig>,
) -> Result<Json<HashMap<String, EntryReference>>, NotFound<Json<ApiError>>> {
    let path = PathBuf::from(&conf.workspace);

    let yaml_file = path
        .join(EntryPaths::quake())
        .join(EntryPaths::references())
        .join(format!("{:}.yml", entry_type));

    let content = match fs::read_to_string(yaml_file) {
        Ok(text) => text,
        Err(err) => {
            return Err(NotFound(Json(ApiError {
                msg: err.to_string(),
            })))
        }
    };

    let refs: HashMap<String, EntryReference> = serde_yaml::from_str(&content).unwrap();
    Ok(Json(refs))
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use crate::quake_rocket;

    #[test]
    fn reference() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let mut response = client.get("/reference/todo").dispatch();

        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        println!("{:}", res);
        assert_eq!(response.status(), Status::Ok);
    }
}
