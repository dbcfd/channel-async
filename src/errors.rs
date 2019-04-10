use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Channel send failed")]
    Send,
    #[fail(display = "Receive failed due to sender disconnected")]
    Disconnected,
    #[fail(display = "Tokio timer error")]
    TokioTimer(#[fail(cause)] tokio_timer::Error),
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}