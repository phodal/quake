#[macro_use]
extern crate log;

use tokio_core::reactor::Core;

pub mod model;
mod main_loop;
mod setup;
mod ui;
mod repository;

fn main() -> Result<(), ()> {
    env_logger::init();
    info!("starting up");

    // todo: ui
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let initial_state = setup::initial_state(handle);
    core.run(initial_state).unwrap();


    Ok(())
}
