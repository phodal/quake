use rocket::Error;
use rocket::fs::{FileServer, relative};

#[allow(unused_imports)]
use action_api::parse_query;
#[allow(unused_imports)]
use entry_api::entry;

mod entry_api;
mod action_api;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub msg: String,
}

#[rocket::main]
pub async fn start_server() -> Result<(), Error> {
    rocket::build()
        .mount("/home", FileServer::from(relative!("quake_webapp")))
        .mount("/entry", routes![entry_api::entry])
        .mount("/action", routes![action_api::parse_query])
        .launch()
        .await
}
