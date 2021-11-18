use tokio_core::reactor::Handle;
use crate::main_loop;

pub(crate) fn initial_state(handle: Handle) -> main_loop::MainLoopState {
    main_loop::MainLoopState {
        _handle: handle
    }
}