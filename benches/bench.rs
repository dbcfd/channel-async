#![feature(futures_api, async_await, await_macro)]
use criterion::{criterion_group, criterion_main, Criterion};
use channel_async;
use futures::{FutureExt, TryFutureExt, TryStreamExt};
use std::time::Duration;

fn bench_unbounded(c: &mut Criterion) {
    let benchmark = criterion::Benchmark::new(
        "10",
        |b| {
            let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

            b.iter(|| {
                let (tx, rx) = channel_async::unbounded(Duration::from_millis(10));

                let send_fut = async move {
                    for i in 0..100 {
                        await!(tx.send(i)).expect("Failed to send");
                    }
                };

                let recv_fut = async move {
                    let f = rx.try_fold(vec![], |mut agg, x| {
                        agg.push(x);
                        futures::future::ready(Ok(agg))
                    });
                    await!(f)
                };

                rt.spawn(send_fut.unit_error().boxed().compat());

                let recv = rt.block_on(recv_fut.boxed().compat()).expect("Failed to receive");

                assert_eq!(recv.len(), 100);
            });
        }
    );

    c.bench("channel",
            benchmark
                .sample_size(50)
                .nresamples(1)
                .measurement_time(std::time::Duration::from_secs(15))
    );

    let benchmark = criterion::Benchmark::new(
        "100",
        |b| {
            let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

            b.iter(|| {
                let (tx, rx) = channel_async::unbounded(Duration::from_millis(100));

                let send_fut = async move {
                    for i in 0..100 {
                        await!(tx.send(i)).expect("Failed to send");
                    }
                };

                let recv_fut = async move {
                    let f = rx.try_fold(vec![], |mut agg, x| {
                        agg.push(x);
                        futures::future::ready(Ok(agg))
                    });
                    await!(f)
                };

                rt.spawn(send_fut.unit_error().boxed().compat());

                let recv = rt.block_on(recv_fut.boxed().compat()).expect("Failed to receive");

                assert_eq!(recv.len(), 100);
            });
        }
    );

    c.bench("channel",
            benchmark
                .sample_size(50)
                .nresamples(1)
                .measurement_time(std::time::Duration::from_secs(15))
    );
}

criterion_group!(benches, bench_unbounded);

//
// Benchmark: cargo bench -- --verbose
// aggregation_filter_map/4sics
// time:   [448.23 ms 448.41 ms 448.23 ms]
// slope  [448.23 ms 448.23 ms] R^2            [0.9657065 0.9657065]
// mean   [451.14 ms 451.14 ms] std. dev.      [19.384 ms 19.384 ms]
// median [449.64 ms 449.64 ms] med. abs. dev. [21.228 ms 21.228 ms]
// aggregation_push/4sics  time:   [446.97 ms 441.84 ms 446.97 ms]
// slope  [446.97 ms 446.97 ms] R^2            [0.9566896 0.9566896]
// mean   [436.28 ms 436.28 ms] std. dev.      [8.5860 ms 8.5860 ms]
// median [434.86 ms 434.86 ms] med. abs. dev. [8.0647 ms 8.0647 ms]
criterion_main!(benches);