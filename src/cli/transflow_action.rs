use quake_core::QuakeConfig;
use std::error::Error;

pub fn transflow_action(action: String, _conf: &QuakeConfig) -> Result<(), Box<dyn Error>> {
    if action.as_str() == "define" {}
    Ok(())
}
