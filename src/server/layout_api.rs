use std::fs;
use std::path::PathBuf;

use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::{get, State};

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::quake::SimpleLayout;
use quake_core::QuakeConfig;

use crate::server::ApiError;

#[get("/dashboard")]
pub fn dashboard_layout(
    config: &State<QuakeConfig>,
) -> Result<Json<SimpleLayout>, NotFound<Json<ApiError>>> {
    let workspace = PathBuf::from(config.workspace.clone());
    let path = workspace
        .join(EntryPaths::quake())
        .join(EntryPaths::dashboard_layout());

    let content = fs::read_to_string(path).unwrap();
    let layout = match SimpleLayout::from_text(content.as_str()) {
        Ok(layout) => layout,
        Err(err) => {
            return Err(NotFound(Json(ApiError {
                msg: err.to_string(),
            })))
        }
    };

    Ok(Json(layout))
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use crate::quake_rocket;

    #[test]
    fn dashboard_layout() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let mut response = client.get("/layout/dashboard").dispatch();

        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        println!("{:?}", res);
        assert_eq!(response.status(), Status::Ok);
    }
}
