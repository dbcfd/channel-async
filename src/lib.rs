//! #channel-async
//!
//! Async/stream extensions for crossbeam-channel
//!
//! ```no-run
//! use futures::{FutureExt, TryFutureExt};
//!
//!#[tokio::main]
//!async fn run_channels() {
//!    let (tx, rx) = channel_async::unbounded(Duration::from_millis(100));
//!
//!    let send_fut = async move {
//!        for i in 1..100 {
//!            tx.send(i).await.expect("Failed to send");
//!        }
//!    };
//!
//!    tokio::spawn(send_fut);
//!
//!    let recv_fut = async move {
//!      let rcvd: Vec<_> = rx.collect().await;
//!      rcvd
//!    };
//!
//!    let rcvd = recv_fut.await;
//!
//!    println!("Received {} messages", rcvd.len());
//!}
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

    #[tokio::test]
    async fn send_receive() {
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

        tokio::spawn(send_fut);

        let recv = recv_fut.await;

        assert_eq!(recv.len(), 100);
    }

    #[tokio::test(single_thread)]
    async fn send_receive_slow_sender_single_thread() {
        let (tx, rx) = unbounded(Duration::from_millis(10));

        let send_fut = async move {
            for i in 0..100usize {
                if i % 10 == 0 {
                    tokio_timer::sleep(Duration::from_millis(100)).await;
                }
                tx.send(i).await.expect("Failed to send");
            }
        };

        let recv_fut = async move {
            let f: Vec<_> = rx.collect().await;
            f
        };

        tokio::spawn(send_fut);

        let recv = recv_fut.await;

        assert_eq!(recv.len(), 100);
    }

    #[tokio::test(multi_thread)]
    async fn send_receive_slow_sender_multi_thread() {
        let (tx, rx) = unbounded(Duration::from_secs(10));

        let send_fut = async move {
            for i in 0..100usize {
                if i % 10 == 0 {
                    tokio_timer::sleep(Duration::from_millis(100)).await;
                }
                tx.send(i).await.expect("Failed to send");
            }
        };

        let recv_fut = async move {
            let f: Vec<_> = rx.collect().await;
            f
        };

        tokio::spawn(send_fut);

        let recv = recv_fut.await;

        assert_eq!(recv.len(), 100);
    }
}
