use std::fs;
use std::path::PathBuf;

use figment::providers::Yaml;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Format, Serialized};
use rocket::figment::{Figment, Profile};
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket::{routes, Build, Config, Rocket, State};
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde_derive::{Deserialize, Serialize};

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
use quake_core::QuakeConfig;

mod action_api;
mod entry_api;
mod helper;
mod layout_api;
mod processor_api;
mod reference_api;
mod transflow_api;

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

pub fn quake_rocket() -> Rocket<Build> {
    // let cors = cors();
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Yaml::file(".quake.yaml"))
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("workspace", "."))
        .select(Profile::from_env_or("server_location", "web"));

    let server: String = figment.extract_inner("server_location").unwrap();
    rocket::custom(figment)
        .mount("/", FileServer::from(server))
        .mount(
            "/entry",
            routes![
                entry_api::get_entry_defines,
                entry_api::get_entry,
                entry_api::create_entry,
                entry_api::update_entry
            ],
        )
        .mount(
            "/action",
            routes![action_api::parse_query, action_api::suggest],
        )
        .mount(
            "/transflow",
            routes![
                transflow_api::transflow_gen_code,
                transflow_api::load_custom_transfuncs,
                transflow_api::translate
            ],
        )
        .mount("/reference", routes![reference_api::reference_by_type])
        .mount(
            "/processor",
            routes![processor_api::lookup_file, processor_api::upload],
        )
        .mount("/layout", routes![layout_api::dashboard_layout])
        .attach(AdHoc::config::<QuakeConfig>())
    // .attach(cors().to_cors().unwrap())
}

#[allow(dead_code)]
fn cors() -> CorsOptions {
    CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
}

pub fn defines(config: &State<QuakeConfig>) -> EntryDefines {
    let path = PathBuf::from(config.workspace.clone()).join(EntryPaths::entries_define());
    let defines: EntryDefines = serde_yaml::from_str(&*fs::read_to_string(path).unwrap()).unwrap();
    defines
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use super::quake_rocket;

    #[test]
    fn hello_world() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_todo_entry() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let response = client.get("/entry/todo/1").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "{\"title\":\"time support\",\"author\":\"\",\"created_date\":\"2021-11-24 19:14:10\",\"updated_date\":\"2021-11-24 19:14:10\",\"id\":1,\"content\":\"\\n\\nahaha\\n\"}")
    }
}
