use rocket::{Config, Error};
use rocket::fairing::AdHoc;
use rocket::figment::{Figment, Profile};
use rocket::figment::providers::{Env, Format, Serialized, Toml};
use rocket::fs::FileServer;

#[allow(unused_imports)]
use action_api::parse_query;
#[allow(unused_imports)]
use entry_api::{create_entry, get_entries, get_entry, update_entry};

mod entry_api;
mod action_api;
mod search_api;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiString {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSuccess {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuakeServerConfig {
    pub workspace: String,
    pub search_url: String,
    pub server_location: String,
}

impl Default for QuakeServerConfig {
    fn default() -> Self {
        QuakeServerConfig {
            workspace: "".to_string(),
            search_url: "".to_string(),
            server_location: "quake_webapp".to_string()
        }
    }
}

#[rocket::main]
pub async fn start_server() -> Result<(), Error> {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file("QuakeServer.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("workspace", "."))
        .select(Profile::from_env_or("search_url", "http://127.0.0.1:7700"));
        .select(Profile::from_env_or("server_location", "web"));


    let server: String = figment.extract_inner("server_location").unwrap();
    rocket::custom(figment)
        .mount("/", FileServer::from(server))
        .mount("/entry", routes![
            entry_api::get_entries,
            entry_api::get_entries_csv,
            entry_api::get_entry,
            entry_api::create_entry,
            entry_api::update_entry
        ])
        .mount("/action", routes![action_api::parse_query, action_api::suggest])
        .attach(AdHoc::config::<QuakeServerConfig>())
        .launch()
        .await
}
