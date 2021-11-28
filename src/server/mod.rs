use std::path::PathBuf;

use rocket::Error;
use rocket::fs::{FileServer, relative};

use quake_core::input_parser::InputParser;

use crate::action::entry_sets::Entrysets;

#[get("/<entry_type>")]
fn entry(entry_type: &str) -> String {
    let path = PathBuf::from("_fixtures").join(entry_type);
    let result = Entrysets::jsonify(&path);
    result.unwrap()
}

#[get("/<input>")]
fn parse_query(input: &str) -> String {
    let parser = InputParser::from(input);
    serde_json::to_string(&parser).unwrap()
}

#[rocket::main]
pub async fn start_server() -> Result<(), Error> {
    rocket::build()
        .mount("/home", FileServer::from(relative!("quake_webapp")))
        .mount("/entry", routes![entry])
        .mount("/search", routes![parse_query])
        .launch()
        .await
}