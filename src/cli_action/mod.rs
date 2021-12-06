use std::error::Error;

use quake_core::quake::QuakeAction;
use quake_core::QuakeConfig;

pub mod entry_action;
pub mod helper;
pub mod quake_action;
pub mod transflow_action;

pub fn action(expr: QuakeAction, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    match expr.object.as_str() {
        "quake" => quake_action::quake_action(expr.action, &conf),
        "flow" => transflow_action::transflow_action(expr.action, &conf),
        "transflow" => Ok(()),
        _ => entry_action::entry_action(&expr, conf),
    }
}
