//! #channel-async
//!
//! Async/stream extensions for crossbeam-channel
mod errors;
mod receiver;
mod sender;

pub use errors::Error;
pub use receiver::Receiver;
pub use sender::Sender;

use std::time::Duration;

pub fn unbounded<T>(delay: Duration) -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = crossbeam_channel::unbounded();
    (Sender::new(tx, delay), Receiver::new(rx, delay))
}

pub fn bounded<T>(delay: Duration, cap: usize) -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = crossbeam_channel::bounded(cap);
    (Sender::new(tx, delay), Receiver::new(rx, delay))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[test]
    fn send_receive() {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

        let (tx, rx) = unbounded(Duration::from_millis(100));

        let send_fut = async move {
            for i in 0..100usize {
                tx.send(i).await.expect("Failed to send");
            }
        };

        let recv_fut = async move {
            let f: Vec<_> = rx.collect().await;
            f
        };

        rt.spawn(send_fut);

        let recv = rt.block_on(recv_fut);

        assert_eq!(recv.len(), 100);
    }
}
