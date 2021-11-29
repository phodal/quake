use rocket::response::content;
use rocket::response::content::Json;
use rocket::tokio::task::spawn_blocking;

use crate::server::ApiError;

#[get("/<entry_type>", rank = 3)]
pub(crate) async fn get_entries(entry_type: &str) -> Json<String> {
    let request_url = format!("http://127.0.0.1:7700/indexes/{:}/documents", entry_type);

    let vec = spawn_blocking (|| content::Json(
        reqwest::blocking::get(request_url)
            .unwrap()
            .text()
            .unwrap(),
    )).await.map_err(|e| ApiError {
        msg: format!("{:?}", e)
    }).unwrap();

    vec
}
