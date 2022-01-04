use std::error::Error;

use quake_core::quake::QuakeActionNode;
use quake_core::QuakeConfig;

pub mod entry_action;
pub mod helper;
pub mod quake_action;

pub fn action(expr: QuakeActionNode, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    match expr.entry.as_str() {
        "quake" => quake_action::quake_action(expr.action, &conf),
        _ => entry_action::entry_action(&expr, conf),
    }
}
