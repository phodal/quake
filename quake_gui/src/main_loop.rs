use futures::Future;
use tokio_core::reactor::Handle;

pub(crate) struct MainLoopState {
    pub(crate) _handle: Handle,
}

impl Future for MainLoopState {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        loop {

        }
    }
}