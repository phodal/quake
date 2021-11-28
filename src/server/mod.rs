use std::path::PathBuf;

use rocket::Error;
use rocket::fs::{FileServer, relative};

use crate::action::entry_sets::Entrysets;

#[get("/<entry_type>")]
fn entry(entry_type: &str) -> String {
    let path = PathBuf::from("_fixtures").join(entry_type);
    let result = Entrysets::jsonify(&path);
    result.unwrap()
}

#[rocket::main]
pub async fn start_server() -> Result<(), Error> {
    rocket::build()
        .mount("/", FileServer::from(relative!("quake_webapp")))
        .mount("/entry", routes![entry])
        .launch()
        .await
}