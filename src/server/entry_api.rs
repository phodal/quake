use std::collections::HashMap;
use std::error::Error;
use rocket::response::content;
use rocket::response::content::Json;

#[get("/<entry_type>", rank = 3)]
pub(crate) async fn entry(entry_type: &str) -> Json<String> {
    let request_url = format!("http://127.0.0.1:7700/indexes/{:}/documents", entry_type);

    content::Json(
        reqwest::blocking::get(request_url)
            .unwrap()
            .text()
            .unwrap(),
    )
}