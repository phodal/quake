use std::fs;
use std::path::PathBuf;

use rocket::response::content;
use rocket::response::content::Json;
use rocket::State;
use rocket::tokio::task::spawn_blocking;

use quake_core::entry::{EntryDefine, EntryDefineFile};
use quake_core::action_parser::ActionDefine;

use crate::server::{ApiError, QuakeServerConfig};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ActionSuggest {
    pub entries: Vec<EntryDefine>,
    pub actions: Vec<String>,
}

impl Default for ActionSuggest {
    fn default() -> Self {
        ActionSuggest { entries: vec![], actions: vec![] }
    }
}

#[get("/query?<input>")]
pub fn parse_query(input: String) -> String {
    let result = ActionDefine::from(input.as_str());
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

#[get("/suggest")]
pub async fn suggest(config: &State<QuakeServerConfig>) -> Json<String> {
    let mut suggest = ActionSuggest::default();
    let path = PathBuf::from(&config.workspace).join("entries-define.yaml");

    suggest.entries = spawn_blocking(|| {
        let entries_str = fs::read_to_string(path).expect("cannot read entries-define.yaml");
        let entries: EntryDefineFile = serde_yaml::from_str(&*entries_str).unwrap();
        entries.entries
    }).await.map_err(|e| ApiError {
        msg: format!("{:?}", e)
    }).unwrap();

    let actions = vec!["add", "edit", "sync", "dump"];
    for action in actions {
        suggest.actions.push(action.to_string());
    }

    let string = serde_json::to_string(&suggest).unwrap();
    content::Json(string)
}
