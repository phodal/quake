use std::fs;
use std::option::Option::None;
use std::path::PathBuf;

use rocket::fs::NamedFile;
use rocket::response::content::JavaScript;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::{get, post};

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
use quake_core::quake::QuakeTransflowNode;
use quake_core::transflow::js_flow_codegen::JsFlowCodegen;
use quake_core::transflow::Transflow;
use quake_core::QuakeConfig;

use crate::server::ApiError;
use quake_core::usecases::flow_usecases;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FlowRequest {
    flow: String,
}

/// create temp transflow to show element
#[post("/translate/<name>", data = "<flow>")]
pub(crate) async fn translate(
    name: String,
    flow: Json<FlowRequest>,
    config: &State<QuakeConfig>,
) -> Result<JavaScript<String>, Json<ApiError>> {
    let format = format!("transflow {:} {{ {:} }}", name, flow.flow);
    let node = match QuakeTransflowNode::from_text(format.as_str()) {
        Ok(node) => node,
        Err(err) => {
            return Err(Json(ApiError {
                msg: err.to_string(),
            }));
        }
    };

    let path = PathBuf::from(config.workspace.clone()).join(EntryPaths::entries_define());

    let content = fs::read_to_string(path).unwrap();
    let defines: EntryDefines = serde_yaml::from_str(&*content).unwrap();

    let flow = Transflow::from(defines.entries, node);
    //
    let trans = JsFlowCodegen::gen_transform(&flow, None);
    let elements = JsFlowCodegen::gen_element(&flow, None);

    let scripts = format!("{:} \n{:}", trans.join("\n"), elements.join("\n"));

    Ok(JavaScript(scripts))
}

/// todo: load transflow from yaml files
/// 1. load yaml file with define
/// 2. generate js scripts
/// 3. create router
/// as nocode
#[get("/script/gen_code.js")]
pub(crate) async fn transflow_gen_code(
    config: &State<QuakeConfig>,
) -> Result<JavaScript<String>, Json<ApiError>> {
    let path = PathBuf::from(config.workspace.clone());
    let scripts = match flow_usecases::dump_flows(path) {
        Ok(scripts) => scripts,
        Err(err) => {
            return Err(Json(ApiError {
                msg: err.to_string(),
            }))
        }
    };
    Ok(JavaScript(scripts))
}

/// todo: load transflow from yaml files
/// as lowcode
#[get("/script/custom_transfuncs.js")]
pub(crate) async fn load_custom_transfuncs(config: &State<QuakeConfig>) -> Option<NamedFile> {
    let path = PathBuf::from(config.workspace.clone());
    let fs = path
        .join(EntryPaths::quake())
        .join(EntryPaths::transfuncs());

    let file = NamedFile::open(fs);
    file.await.ok()
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use crate::quake_rocket;

    #[test]
    fn transflow_translate_script() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let url = format!("/transflow/translate/{:}", "show_timeline");
        let flow = "from('todo','blog').to(<quake-calendar>)";
        let body = format!("{{ \"flow\": {:?} }}", flow);
        let response = client.post(url).body(body).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn transflow_script() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let mut response = client.get("/transflow/script/gen_code.js").dispatch();

        let mut res = "".to_string();
        let _ = response.read_to_string(&mut res);

        assert_eq!(response.status(), Status::Ok);
    }
}
