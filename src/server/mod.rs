use rocket::{Config, Error};
use rocket::fairing::AdHoc;
use rocket::figment::{Figment, Profile};
use rocket::figment::providers::{Env, Format, Serialized, Toml};
use rocket::fs::{FileServer, relative};

#[allow(unused_imports)]
use action_api::parse_query;
#[allow(unused_imports)]
use entry_api::{get_entries, get_entry, create_entry, update_entry};

mod entry_api;
mod action_api;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiString {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuakeServerConfig {
    pub workspace: String
}

impl Default for QuakeServerConfig {
    fn default() -> Self {
        QuakeServerConfig { workspace: "".to_string() }
    }
}

#[rocket::main]
pub async fn start_server() -> Result<(), Error> {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file("QuakeServer.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("workspace", "."));

    // todo: loading from figment config
    let path = relative!("quake_webapp");
    rocket::custom(figment)
        .mount("/", FileServer::from(path))
        .mount("/entry", routes![entry_api::get_entries, entry_api::get_entry, entry_api::create_entry, entry_api::update_entry])
        .mount("/action", routes![action_api::parse_query, action_api::suggest])
        .attach(AdHoc::config::<QuakeServerConfig>())
        .launch()
        .await
}
