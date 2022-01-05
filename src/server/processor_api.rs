use std::path::PathBuf;

use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::{get, State};

use quake_core::QuakeConfig;

use crate::server::ApiError;
use crate::usecases::processor_usecases::get_file_property_path;

#[get("/<entry_type>?<file_prop>")]
pub(crate) async fn lookup_file(
    entry_type: String,
    file_prop: String,
    conf: &State<QuakeConfig>,
) -> Result<NamedFile, NotFound<Json<ApiError>>> {
    let path = PathBuf::from(&conf.workspace).join(entry_type);
    let path_buf = get_file_property_path(&file_prop, &path);

    if !path_buf.exists() {
        return Err(NotFound(Json(ApiError {
            msg: format!("cannot find file: {:}", path_buf.display()),
        })));
    }

    let file = NamedFile::open(path_buf);
    Ok(file.await.ok().unwrap())
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
        let mut response = client
            .get("/processor/papers?file_prop=entries.csv")
            .dispatch();

        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        assert_eq!(response.status(), Status::Ok);
    }
}
