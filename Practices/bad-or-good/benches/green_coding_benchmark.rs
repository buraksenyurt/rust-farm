use bad_or_good::Fibonacci;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::time::Duration;

fn fibonacci_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci Benchmark");
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10);

    for i in 36..=40 {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
            b.iter(|| Fibonacci::calc_worst(black_box(i)));
        });
    }

    for i in 36..=40 {
        group.bench_with_input(BenchmarkId::from_parameter(i +10), &i, |b, &i| {
            let mut memo_set: HashMap<u64, u64> = HashMap::new();
            b.iter(|| Fibonacci::calc_green(black_box(i), black_box(&mut memo_set)));
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = fibonacci_benchmark
}
criterion_main!(benches);
