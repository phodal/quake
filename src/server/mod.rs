use std::path::PathBuf;

use rocket::Error;
use rocket::fs::{FileServer, relative};

use entry_api::entry;
use quake_core::input_parser::InputParser;

mod entry_api;

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
        .mount("/entry", routes![entry_api::entry])
        .mount("/search", routes![parse_query])
        .launch()
        .await
}
