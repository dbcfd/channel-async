#![feature(async_await)]
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
// Benchmark: cargo bench --verbose
// Benchmarking channel/10: AnalyzingCriterion.rs
// channel/10              time:   [1.0201 ms 925.34 us 1.0201 ms]
//
// Benchmarking channel/100: AnalyzingCriterion.rs
// channel/100             time:   [6.7099 ms 7.1574 ms 6.7099 ms]
criterion_main!(benches);