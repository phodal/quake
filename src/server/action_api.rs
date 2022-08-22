use rocket::get;
use rocket::serde::json::Json;
use rocket::State;

use quake_core::parser::quake::QuakeActionNode;
use quake_core::QuakeConfig;

use crate::server::ApiError;
use crate::usecases::suggest_usecases;
use crate::usecases::suggest_usecases::ActionSuggest;

#[get("/query?<input>")]
pub fn parse_query(input: String) -> String {
    let result = QuakeActionNode::from_text(input.as_str());

    match result {
        Ok(value) => serde_json::to_string(&value).unwrap(),
        Err(err) => serde_json::to_string(&ApiError {
            msg: format!("{:?}", err),
        })
        .unwrap(),
    }
}

#[get("/suggest")]
pub async fn suggest(config: &State<QuakeConfig>) -> Json<ActionSuggest> {
    let workspace = &config.workspace;
    let suggest = suggest_usecases::create_suggest(workspace);
    Json(suggest)
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use crate::quake_rocket;

    #[test]
    fn todo_show_should_return_json() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let mut response = client.get("/action/query?input=todo.show").dispatch();

        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            "{\"entry\":\"todo\",\"action\":\"show\",\"text\":\"\",\"parameters\":[]}",
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
