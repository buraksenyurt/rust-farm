use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fizz_bench::*;

pub fn fizz_buzz_enum_benchmark(c: &mut Criterion) {
    c.bench_function("Fizz Buzz with Enum", |b| {
        b.iter(|| fizz_buzz_v2(black_box(15)))
    });
}

pub fn fizz_buzz_string_benchmark(c: &mut Criterion) {
    c.bench_function("Fizz Buzz with String", |b| {
        b.iter(|| fizz_buzz_v1(black_box(15)))
    });
}

criterion_group!(
    benches,
    fizz_buzz_enum_benchmark,
    fizz_buzz_string_benchmark
);
criterion_main!(benches);
