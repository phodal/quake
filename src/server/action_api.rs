use quake_core::input_parser::InputParser;
use crate::server::ApiError;

#[get("/query?<input>")]
pub fn parse_query(input: String) -> String {
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
