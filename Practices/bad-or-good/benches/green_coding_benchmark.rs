use bad_or_good::Fibonacci;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;
use std::time::Duration;

fn fibonacci_benchmarks(c: &mut Criterion) {
    let mut worst_case_group = c.benchmark_group("Worst Case");
    worst_case_group.measurement_time(Duration::from_secs(20));
    worst_case_group.sample_size(10);

    for i in 36..=40 {
        worst_case_group.bench_with_input(BenchmarkId::new("Worst", i), &i, |b, &i| {
            b.iter(|| Fibonacci::calc_worst(black_box(i)));
        });
    }
    worst_case_group.finish();

    let mut green_case_group = c.benchmark_group("Green Case");
    green_case_group.measurement_time(Duration::from_secs(20));
    green_case_group.sample_size(10);

    for i in 36..=40 {
        green_case_group.bench_with_input(BenchmarkId::new("Green", i), &i, |b, &i| {
            let mut memo_set: HashMap<u64, u64> = HashMap::new();
            b.iter(|| Fibonacci::calc_green(black_box(i), black_box(&mut memo_set)));
        });
    }
    green_case_group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = fibonacci_benchmarks
}
criterion_main!(benches);
