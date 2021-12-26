use std::option::Option::None;
use wasm_bindgen::prelude::*;

use quake_core::entry::EntryDefines;
use quake_core::quake::{QuakeActionNode, QuakeTransflowNode, SimpleLayout};
use quake_core::transflow::js_flow_codegen::JsFlowCodegen;
use quake_core::transflow::Transflow;

#[wasm_bindgen]
pub fn parse_transflow(string: &str) -> String {
    let flow = QuakeTransflowNode::from_text(string).unwrap();
    serde_json::to_string(&flow).unwrap()
}

#[wasm_bindgen]
pub fn parse_action(string: &str) -> String {
    let node = QuakeActionNode::from_text(string).unwrap();
    serde_json::to_string(&node).unwrap()
}

#[wasm_bindgen]
pub fn parse_layout(string: &str) -> String {
    let layout = SimpleLayout::from_text(string).unwrap();
    serde_json::to_string(&layout).unwrap()
}

#[wasm_bindgen]
pub fn flow_to_code(content: &str, defines: &str) -> String {
    let flow = QuakeTransflowNode::from_text(content).unwrap();
    let defines: EntryDefines = serde_json::from_str(defines).unwrap();

    let flow = Transflow::from(defines.entries, flow);

    let trans = JsFlowCodegen::gen_transform(&flow, &None);
    let elements = JsFlowCodegen::gen_element(&flow, &None);

    let scripts = format!("{:} \n{:}", trans.join("\n"), elements.join("\n"));

    scripts
}

#[cfg(test)]
mod tests {
    use crate::flow_to_code;

    #[test]
    fn should_create_flow() {
        let code = "transflow show_calendar { from('todo').to(<quake-calendar>); }";
        let defines = r#"{
  "entries": [{
      "type": "todo",
      "display": "",
      "properties": [
        {
          "title": "Title"
        },
        {
          "author": "String"
        },
        {
          "content": "Body"
        },
        {
          "created_date": "Date"
        },
        {
          "updated_date": "Date"
        }
      ]
    }]}"#;
        let code = flow_to_code(code, defines);
        assert!(code.contains("from_todo_to_quake_calendar"));
    }
}
