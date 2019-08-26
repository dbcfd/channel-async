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

    pub fn into_inner(self) -> crossbeam_channel::Receiver<T> {
        self.inner
    }

}

impl<T: Send + 'static> Stream for Receiver<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self {
            inner,
            delay,
            pending,
        } = unsafe { self.get_unchecked_mut() };
        loop {
            match pending {
                None => match inner.try_recv() {
                    Err(crossbeam_channel::TryRecvError::Disconnected) => return Poll::Ready(None),
                    Err(crossbeam_channel::TryRecvError::Empty) => {
                        *pending = Some(tokio_timer::sleep(*delay));
                    }
                    Ok(v) => return Poll::Ready(Some(v)),
                },
                Some(pending_value) => {
                    let pin_pending = unsafe { Pin::new_unchecked(pending_value) };
                    futures::ready!(pin_pending.poll(cx));
                    *pending = None;
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
        let (_, _r) = crate::unbounded::<i32>(Duration::from_millis(100));

        assert_send::<Receiver<i32>>();
        assert_sync::<Receiver<i32>>();
    }
}
