#[macro_use]
extern crate log;

use std::fs::File;

use daemonize::Daemonize;
use tokio_core::reactor::Core;

pub mod model;
mod main_loop;
mod setup;


fn main() -> Result<(), ()> {
    env_logger::init();
    info!("starting up");

    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/quake.pid")
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");


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
