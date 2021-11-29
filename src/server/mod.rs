use rocket::{Config, Error};
use rocket::fairing::AdHoc;
use rocket::figment::{Figment, Profile};
use rocket::figment::providers::{Env, Format, Serialized, Toml};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct QuakeServerConfig {

}

#[rocket::main]
pub async fn start_server() -> Result<(), Error> {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file("App.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("APP_PROFILE", "default"));


    rocket::custom(figment)
        .mount("/", FileServer::from(relative!("quake_webapp")))
        .mount("/entry", routes![entry_api::entry])
        .mount("/action", routes![action_api::parse_query, action_api::suggest])
        .attach(AdHoc::config::<QuakeServerConfig>())
        .launch()
        .await
}
