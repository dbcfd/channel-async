# channel-async

[![build status][travis-badge]][travis-url]
[![crates.io version][crates-badge]][crates-url]
[![docs.rs docs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

Async/stream extensions to [crossbeam-channel](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel) on top of [Futures 0.3](https://github.com/rust-lang-nursery/futures-rs) Stream. It is primarily intended for usage with [Tokio](https://github.com/tokio-rs/tokio).

[Documentation](https://docs.rs/channel-async/latest/)

[travis-badge]: https://travis-ci.com/dbcfd/channel-async.svg?branch=master
[travis-url]: https://travis-ci.com/dbcfd/channel-async
[crates-badge]: https://img.shields.io/crates/v/channel-async.svg?style=flat-square
[crates-url]: https://crates.io/crates/channel-async
[docs-badge]: https://img.shields.io/badge/docs.rs-latest-blue.svg?style=flat-square
[docs-url]: https://docs.rs/channel-async
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square
[mit-url]: LICENSE-MIT

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
channel-async = "0.3.0-alpha.5"
```

Next, add this to your crate:

```rust
use futures::{FutureExt, TryFutureExt};

#[tokio::main]
async fn run_channels() {
    let (tx, rx) = channel_async::unbounded(Duration::from_millis(100));

    let send_fut = async move {
        for i in 1..100 {
            tx.send(i).await.expect("Failed to send");
        }
    };
    
    tokio::spawn(send_fut);

    let recv_fut = async move {
      let rcvd: Vec<_> = rx.collect().await;
      rcvd
    };
    
    let rcvd = recv_fut.await;
    
    println!("Received {} messages", rcvd.len());
}
```

## License

This project is licensed under the [MIT license](./LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in tls-async by you, shall be licensed as MIT, without any additional
terms or conditions.
