use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Channel send failed")]
    Send,
    #[fail(display = "Receive failed due to sender disconnected")]
    Disconnected,
    #[fail(display = "Tokio timer error")]
    TokioTimer(#[fail(cause)] tokio_timer::Error),
    #[fail(display = "Receiver was not in a state to clone")]
    Clone,
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}
