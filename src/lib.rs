//! #channel-async
//!
//! Async/stream extensions for crossbeam-channel
#![feature(async_await)]
mod errors;
mod receiver;
mod sender;

pub use errors::Error as Error;
pub use receiver::Receiver as Receiver;
pub use sender::Sender as Sender;

use std::time::Duration;

pub fn unbounded<T>(delay: Duration) -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = crossbeam_channel::unbounded();
    (
        Sender::new(tx, delay),
        Receiver::new(rx, delay),
        )
}

pub fn bounded<T>(delay: Duration, cap: usize) -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = crossbeam_channel::bounded(cap);
    (
        Sender::new(tx, delay),
        Receiver::new(rx, delay),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{FutureExt, TryStreamExt, TryFutureExt};

    #[test]
    fn send_receive() {
        let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

        let (tx, rx) = unbounded(Duration::from_millis(100));

        let send_fut = async move {
            for i in 0..100usize {
                tx.send(i).await.expect("Failed to send");
            }
        };

        let recv_fut = async move {
            let f = rx.try_fold(vec![], |mut agg, x| {
                agg.push(x);
                futures::future::ready(Ok(agg))
            });
            f.await
        };

        rt.spawn(send_fut.unit_error().boxed().compat());

        let recv = rt.block_on(recv_fut.boxed().compat()).expect("Failed to receive");

        assert_eq!(recv.len(), 100);
    }
}

