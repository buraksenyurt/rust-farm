use bad_or_good::Sorting;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn create_data_set(size: usize, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..=max)).collect()
}

fn sorting_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Benchmarks");
    for size in [10, 10_000, 50_000].iter() {
        let max_value = 10_000;
        let data = create_data_set(*size, max_value);

        group.bench_function(format!("Radix Sort {}", size), |b| {
            b.iter(|| {
                let mut data_clone = data.clone();
                Sorting::radix(black_box(&mut data_clone));
            })
        });

        group.bench_function(format!("Bubble Sort {}", size), |b| {
            b.iter(|| {
                let mut data_clone = data.clone();
                Sorting::bubble(black_box(&mut data_clone));
            })
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = sorting_benchmarks
}
criterion_main!(benches);
