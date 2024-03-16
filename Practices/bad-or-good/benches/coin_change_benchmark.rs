use bad_or_good::{Coin, CoinChange};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

fn coin_change_benchmarks(c: &mut Criterion) {
    let mut worst_case_group = c.benchmark_group("Coin Change Worst Case");
    worst_case_group.measurement_time(Duration::from_secs(20));
    worst_case_group.sample_size(10);
    let coins = vec![
        Coin::Penny as i32,
        Coin::Nickel as i32,
        Coin::Dime as i32,
        Coin::Quarter as i32,
    ];

    for amount in [1, 17, 41, 63, 72] {
        worst_case_group.bench_with_input(BenchmarkId::new("Worst", amount), &amount, |b, &i| {
            b.iter(|| CoinChange::calc_worst(black_box(&coins), black_box(i)));
        });
    }
    worst_case_group.finish();

    let mut green_case_group = c.benchmark_group("Coin Change Green Case");
    green_case_group.measurement_time(Duration::from_secs(10));
    green_case_group.sample_size(10);

    for amount in [1, 17, 41, 63, 72] {
        green_case_group.bench_with_input(BenchmarkId::new("Green", amount), &amount, |b, &i| {
            let mut memo_set = vec![-1; (i + 1) as usize];
            memo_set[0] = 0;
            b.iter(|| {
                CoinChange::calc_green(black_box(&coins), black_box(i), black_box(&mut memo_set))
            });
        });
    }
    green_case_group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = coin_change_benchmarks
}
criterion_main!(benches);
