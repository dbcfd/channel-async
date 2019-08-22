use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio_timer::Delay;

pub struct Receiver<T> {
    inner: crossbeam_channel::Receiver<T>,
    delay: Duration,
    pending: Option<Delay>,
}

impl<T> Receiver<T> {
    pub fn new(r: crossbeam_channel::Receiver<T>, delay: Duration) -> Receiver<T> {
        Receiver {
            inner: r,
            delay: delay,
            pending: None,
        }
    }

    fn pending(self: Pin<&mut Self>) -> Pin<&mut Option<Delay>> {
        unsafe { Pin::map_unchecked_mut(self, |x| &mut x.pending) }
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            delay: self.delay,
            pending: None,
        }
    }
}

impl<T: Send + 'static> Stream for Receiver<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match self.as_mut().pending().as_pin_mut() {
                None => match self.inner.try_recv() {
                    Err(crossbeam_channel::TryRecvError::Disconnected) => return Poll::Ready(None),
                    Err(crossbeam_channel::TryRecvError::Empty) => {
                        *self.as_mut().pending().get_mut() = Some(tokio_timer::sleep(self.delay));
                    }
                    Ok(v) => return Poll::Ready(Some(v)),
                },
                Some(pending) => {
                    futures::ready!(pending.poll(cx));
                    *self.as_mut().pending().get_mut() = None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn assert_contracts() {
        let (_, r) = crate::unbounded::<i32>(Duration::from_millis(100));

        let _r2 = r.clone();

        assert_send::<Receiver<i32>>();
        assert_sync::<Receiver<i32>>();
    }
}
