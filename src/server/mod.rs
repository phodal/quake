use std::path::PathBuf;
use rocket::Error;
use crate::action::entry_sets::Entrysets;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/entry/<entry_type>")]
fn entry(entry_type: &str) -> String {
    let path = PathBuf::from("_fixtures").join(entry_type);
    let result = Entrysets::jsonify(&path);
    result.unwrap()
}

#[allow(dead_code)]
#[post("/entry/<entry_type>")]
fn entry_create(entry_type: &str) -> String {
    let path = PathBuf::from("_fixtures").join(entry_type);
    let result = Entrysets::jsonify(&path);
    result.unwrap()
}

#[rocket::main]
pub async fn start_server() -> Result<(), Error> {
    rocket::build()
        .mount("/", routes![index, entry])
        .launch()
        .await
}