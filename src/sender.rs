use crate::errors::Error;

use std::time::Duration;

pub struct Sender<T> {
    inner: crossbeam_channel::Sender<T>,
    delay: Duration,
}

impl<T> Sender<T> {
    pub fn new(s: crossbeam_channel::Sender<T>, delay: Duration) -> Sender<T> {
        Sender {
            inner: s,
            delay: delay,
        }
    }

    pub async fn send(&self, msg: T) -> Result<(), (T, Error)> {
        let mut msg = msg;
        loop {
            match self.inner.try_send(msg) {
                Err(crossbeam_channel::TrySendError::Disconnected(v)) => {
                    return Err((v, Error::Disconnected))
                }
                Err(crossbeam_channel::TrySendError::Full(v)) => {
                    tokio_timer::sleep(self.delay).await;
                    msg = v;
                }
                Ok(_) => return Ok(()),
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.inner.is_full()
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender::new(self.inner.clone(), self.delay)
    }
}
