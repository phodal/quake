use std::fs;
use std::path::PathBuf;

use rocket::fs::NamedFile;
use rocket::response::content::JavaScript;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post};

use quake_core::entry::entry_paths::EntryPaths;
use quake_core::entry::EntryDefines;
use quake_core::quake::QuakeTransflowNode;
use quake_core::transflow::js_flow_codegen::JsFlowCodegen;
use quake_core::transflow::Transflow;
use quake_core::QuakeConfig;

use crate::server::ApiError;

#[get("/define")]
pub(crate) async fn transflow_defines(
    config: &State<QuakeConfig>,
) -> Result<Json<Vec<Transflow>>, NotFound<Json<ApiError>>> {
    let path = PathBuf::from(config.workspace.clone());
    let yaml = path.join(EntryPaths::transflows_yaml());

    let content = match fs::read_to_string(yaml) {
        Ok(content) => content,
        Err(err) => {
            return Err(NotFound(Json(ApiError {
                msg: err.to_string(),
            })));
        }
    };

    let flows: Vec<Transflow> = match serde_yaml::from_str(&*content) {
        Ok(content) => content,
        Err(err) => {
            return Err(NotFound(Json(ApiError {
                msg: err.to_string(),
            })));
        }
    };

    Ok(Json(flows))
}

/// create temp transflow to show element
#[post("/temp/<name>", data = "<input>")]
pub(crate) async fn translate(
    name: String,
    input: String,
    config: &State<QuakeConfig>,
) -> Result<JavaScript<String>, Json<ApiError>> {
    let format = format!("transflow {:} {{ {:} }}", name, input);
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
    let trans = JsFlowCodegen::gen_transform(&flow);
    let elements = JsFlowCodegen::gen_element(&flow, None);

    let scripts = format!("{:} \n{:}", trans.join("\n"), elements.join("\n"));

    Ok(JavaScript(scripts))
}

/// saved transflow scripts
#[get("/script")]
pub(crate) async fn transfunc_script(config: &State<QuakeConfig>) -> Option<NamedFile> {
    let path = PathBuf::from(config.workspace.clone());
    let fs = path.join(EntryPaths::transfuncs());

    let file = NamedFile::open(fs);
    file.await.ok()
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use crate::quake_rocket;

    #[cfg(feature = "webserver")]
    #[test]
    fn transflow_script() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let url = format!("/transflow/temp/{:}", "show_timeline");
        let response = client
            .post(url)
            .body("from('todo','blog').to(<quake-calendar>)")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        println!("{:?}", response.body());
    }
}
