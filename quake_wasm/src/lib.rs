use wasm_bindgen::prelude::*;

use quake_core::quake::{QuakeActionNode, QuakeTransflowNode, SimpleLayout};

#[wasm_bindgen]
pub fn parse_transflow(string: &str) {
    QuakeTransflowNode::from_text(string).unwrap();
}

#[wasm_bindgen]
pub fn parse_action(string: &str) {
    QuakeActionNode::from_text(string).unwrap();
}

#[wasm_bindgen]
pub fn parse_layout(string: &str) {
    SimpleLayout::from_text(string).unwrap();
}
