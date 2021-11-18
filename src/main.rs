// use std::fs::File;
// use daemonize::Daemonize;
// use livesplit_hotkey::{Hook, KeyCode};
#[macro_use]
extern crate log;

use daemonize::Daemonize;
use tokio_core::reactor::Core;

pub mod model;
mod main_loop;
mod setup;


fn main() -> Result<(), ()> {
    env_logger::init();
    info!("starting up");

    let daemonize = Daemonize::new()
        .pid_file("/tmp/quake.pid");

    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let initial_state = setup::initial_state(handle);
    core.run(initial_state).unwrap();

    Ok(())
}
