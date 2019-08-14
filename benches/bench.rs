#![feature(async_await)]
use channel_async;
use criterion::{criterion_group, criterion_main, Criterion};
use futures::{FutureExt, StreamExt, TryFutureExt, TryStreamExt};
use std::time::Duration;

fn create_benchmark(name: &str, sz: usize) -> criterion::Benchmark {
    criterion::Benchmark::new(name, move |b| {
        let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

        b.iter(|| {
            let (tx, rx) = channel_async::unbounded(Duration::from_millis(10));

            let send_fut = async move {
                for i in 0..sz {
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

            let recv = rt
                .block_on(recv_fut.boxed().compat())
                .expect("Failed to receive");

            assert_eq!(recv.len(), sz);
        });
    })
}

fn create_multiple_benchmark(name: &str, sz: usize) -> criterion::Benchmark {
    criterion::Benchmark::new(name, move |b| {
        let mut rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

        b.iter(|| {
            let (tx1, rx1) = channel_async::unbounded(Duration::from_millis(10));
            let (tx2, rx2) = channel_async::unbounded(Duration::from_millis(10));

            let send_fut1 = async move {
                for i in 0..sz {
                    tx1.send(i).await.expect("Failed to send");
                }
            };

            rt.spawn(send_fut1.unit_error().boxed().compat());

            let send_fut2 = async move {
                for i in 0..sz {
                    tx2.send(i).await.expect("Failed to send");
                }
            };

            rt.spawn(send_fut2.unit_error().boxed().compat());

            let (tx, rx) = channel_async::unbounded(Duration::from_millis(10));

            let forward_fut = async move {
                let mut rx1 = rx1.fuse();
                let mut rx1_done = false;
                let mut rx2 = rx2.fuse();
                let mut rx2_done = false;
                loop {
                    let e = futures::select! {
                        opt_e = rx1.next() => {
                            if opt_e.is_none() {
                                rx1_done = true;
                            }
                            opt_e
                        },
                        opt_e = rx2.next() => {
                            if opt_e.is_none() {
                                rx2_done = true;
                            }
                            opt_e
                        }
                    };
                    match e {
                        None => break,
                        Some(e) => tx.send(e).await.expect("Failed to send"),
                    }
                }
                if !rx1_done {
                    while let Some(e) = rx1.next().await {
                        tx.send(e).await.expect("Failed to send");
                    }
                }
                if !rx2_done {
                    while let Some(e) = rx2.next().await {
                        tx.send(e).await.expect("Failed to send");
                    }
                }
            };

            rt.spawn(forward_fut.unit_error().boxed().compat());

            let recv_fut = async move {
                let f = rx.try_fold(vec![], |mut agg, x| {
                    agg.push(x);
                    futures::future::ready(Ok(agg))
                });
                f.await
            };

            let recv = rt
                .block_on(recv_fut.boxed().compat())
                .expect("Failed to receive");

            assert_eq!(recv.len(), sz * 2);
        });
    })
}

fn bench_unbounded(c: &mut Criterion) {
    let benchmark = create_benchmark("10", 10);
    c.bench(
        "single",
        benchmark
            .sample_size(50)
            .nresamples(1)
            .measurement_time(std::time::Duration::from_secs(15)),
    );
    let benchmark = create_benchmark("100", 100);
    c.bench(
        "single",
        benchmark
            .sample_size(50)
            .nresamples(1)
            .measurement_time(std::time::Duration::from_secs(15)),
    );
}

fn bench_multiple_unbounded(c: &mut Criterion) {
    let benchmark = create_multiple_benchmark("10", 10);
    c.bench(
        "multiple",
        benchmark
            .sample_size(50)
            .nresamples(1)
            .measurement_time(std::time::Duration::from_secs(15)),
    );
    let benchmark = create_multiple_benchmark("100", 100);
    c.bench(
        "multiple",
        benchmark
            .sample_size(50)
            .nresamples(1)
            .measurement_time(std::time::Duration::from_secs(15)),
    );
}

criterion_group!(benches, bench_unbounded, bench_multiple_unbounded);

//
// Benchmark: cargo bench --verbose
// Benchmarking channel/10: AnalyzingCriterion.rs
// channel/10              time:   [1.0201 ms 925.34 us 1.0201 ms]
//
// Benchmarking channel/100: AnalyzingCriterion.rs
// channel/100             time:   [6.7099 ms 7.1574 ms 6.7099 ms]
criterion_main!(benches);
