use wasm_bindgen::prelude::*;

use quake_core::quake::{QuakeActionNode, QuakeTransflowNode, SimpleLayout};

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
