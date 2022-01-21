use benchy::factorial;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn factorials_benchmark(c: &mut Criterion) {
    c.bench_function("Factorial Function Benchs", |b| {
        b.iter(|| factorial(black_box(2)))
    });
}

criterion_group!(benches, factorials_benchmark);
criterion_main!(benches);
