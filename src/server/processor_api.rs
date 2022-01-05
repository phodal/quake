use quake_core::entry::entry_file::EntryFile;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::{get, State};
use std::fs;

use quake_core::QuakeConfig;

use crate::server;
use crate::server::entry_api::entry_by_id;
use crate::server::ApiError;
use crate::usecases::processor_usecases::lookup_file_prop_from_define;

#[get("/<entry_type>/<entry_id>")]
pub(crate) async fn lookup_file(
    entry_type: String,
    entry_id: usize,
    conf: &State<QuakeConfig>,
) -> Result<String, NotFound<Json<ApiError>>> {
    let defines = server::defines(conf);
    let define = match defines.find(&*entry_type) {
        None => {
            return Err(NotFound(Json(ApiError {
                msg: format!("cannot find entry type: {:}", entry_type),
            })));
        }
        Some(def) => def,
    };

    let prop = lookup_file_prop_from_define(&define);
    let entry_path = entry_by_id(&entry_type, entry_id, &conf)?;

    let string = fs::read_to_string(entry_path).unwrap();
    let entry_file = EntryFile::from(&string, entry_id).unwrap();

    let prop_value = entry_file.property(&prop).unwrap();

    Ok(prop_value)
}

#[cfg(test)]
mod test {
    use crate::quake_rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use std::io::Read;

    #[test]
    fn reference() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let mut response = client.get("/processor/papers/1").dispatch();

        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        assert_eq!("pdca/polaris.pdf", res);
        assert_eq!(response.status(), Status::Ok);
    }
}
