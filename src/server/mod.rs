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


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub msg: String
}

#[get("/query?<input>")]
fn parse_query(input: String) -> String {
    let result = InputParser::from(input.as_str());
    let output = match result {
        Ok(value) => {
            serde_json::to_string(&value).unwrap()
        }
        Err(err) => {
            serde_json::to_string(&ApiError {
                msg: format!("{:?}", err)
            }).unwrap()
        }
    };

    output
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