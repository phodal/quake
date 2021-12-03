use figment::providers::Yaml;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Format, Serialized};
use rocket::figment::{Figment, Profile};
use rocket::fs::FileServer;
use rocket::{Build, Config, Error, Rocket};

#[allow(unused_imports)]
use action_api::parse_query;
#[allow(unused_imports)]
use quake_core::QuakeConfig;

mod action_api;
mod entry_api;
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

#[rocket::main]
pub async fn start_server() -> Result<(), Error> {
    rocket().launch().await
}

fn rocket() -> Rocket<Build> {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Yaml::file(".quake.yaml"))
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("workspace", "."))
        .select(Profile::from_env_or("search_url", "http://127.0.0.1:7700"))
        .select(Profile::from_env_or("server_location", "web"));

    let server: String = figment.extract_inner("server_location").unwrap();
    rocket::custom(figment)
        .mount("/", FileServer::from(server))
        .mount(
            "/entry",
            routes![
                entry_api::get_entries,
                entry_api::get_entries_csv,
                entry_api::get_entries_from_csv,
                entry_api::get_entry,
                entry_api::create_entry,
                entry_api::update_entry
            ],
        )
        .mount(
            "/action",
            routes![action_api::parse_query, action_api::suggest],
        )
        .attach(AdHoc::config::<QuakeConfig>())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    // todo: ignore test for speed
    #[ignore]
    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[ignore]
    #[test]
    fn get_todo_entry() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/entry/todo/1").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "{\"title\":\"time support\",\"author\":\"\",\"content\":\"\",\"created_date\":\"2021-11-24 19:14:10\",\"updated_date\":\"2021-11-24 19:14:10\",\"id\":1,\"content\":\"\\n\\nahaha\\n\"}")
    }
}
