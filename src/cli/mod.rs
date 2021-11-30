use std::error::Error;

use quake_core::action_parser::ActionDefine;
use quake_core::QuakeConfig;

pub mod entry_action;
pub mod quake_action;

pub fn action(expr: ActionDefine, conf: QuakeConfig) -> Result<(), Box<dyn Error>> {
    if expr.object == "quake" {
        return quake_action::quake_action(expr.action, &conf);
    }

    entry_action::entry_action(&expr, conf)
}
