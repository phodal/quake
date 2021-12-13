use std::fs;
use std::path::PathBuf;

use rocket::get;
use rocket::serde::json::Json;
use rocket::tokio::task::spawn_blocking;
use rocket::State;
use serde_derive::{Deserialize, Serialize};

use quake_core::entry::entry_defines::EntryDefines;
use quake_core::entry::EntryDefine;
use quake_core::parser::quake::QuakeActionNode;
use quake_core::QuakeConfig;

use crate::server::ApiError;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ActionSuggest {
    pub entries: Vec<EntryDefine>,
    pub actions: Vec<String>,
}

#[get("/query?<input>")]
pub fn parse_query(input: String) -> String {
    let result = QuakeActionNode::action_from_text(input.as_str());
    let output = match result {
        Ok(value) => serde_json::to_string(&value).unwrap(),
        Err(err) => serde_json::to_string(&ApiError {
            msg: format!("{:?}", err),
        })
        .unwrap(),
    };

    output
}

#[get("/suggest")]
pub async fn suggest(config: &State<QuakeConfig>) -> Json<ActionSuggest> {
    let mut suggest = ActionSuggest::default();
    let path = PathBuf::from(&config.workspace).join("entries-define.yaml");

    suggest.entries = spawn_blocking(|| {
        let entries_str = fs::read_to_string(path).expect("cannot read entries-define.yaml");
        let entries: EntryDefines = serde_yaml::from_str(&*entries_str).unwrap();
        entries.entries
    })
    .await
    .map_err(|e| ApiError {
        msg: format!("{:?}", e),
    })
    .unwrap();

    let actions = vec!["add", "edit", "show"];
    for action in actions {
        suggest.actions.push(action.to_string());
    }

    Json(suggest)
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use std::io::Read;

    use crate::quake_rocket;

    #[test]
    fn todo_show_should_return_json() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let mut response = client.get("/action/query?input=todo.show").dispatch();

        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            "{\"object\":\"todo\",\"action\":\"show\",\"text\":\"\",\"parameters\":[]}",
            res
        );
    }

    #[test]
    fn suggest_should_works() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let mut response = client.get("/action/suggest").dispatch();

        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        assert_eq!(response.status(), Status::Ok);
    }
}
