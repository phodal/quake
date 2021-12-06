use std::error::Error;

use quake_core::quake::QuakeAction;
use quake_core::QuakeConfig;

pub mod entry_action;
pub mod quake_action;
pub mod transflow_action;

pub fn action(expr: QuakeAction, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    if expr.object == "quake" {
        return quake_action::quake_action(expr.action, &conf);
    } else if expr.object == "flow" {
        // todo: add flow.gen
        return transflow_action::transflow_action(expr.action, &conf);
    }

    entry_action::entry_action(&expr, conf)
}
