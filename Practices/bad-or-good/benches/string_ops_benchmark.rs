use bad_or_good::StringOps;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn concatenate_benchmark(c: &mut Criterion) {
    let mut worst_case_group = c.benchmark_group("String Concatenate Worst Case");
    worst_case_group.measurement_time(Duration::from_secs(10));

    let words = vec![
        "#Sustainable Software Engineering. ".to_string(),
        " ".to_string(),
        "##Topics ".to_string(),
        " ".to_string(),
        "## Building Blocks ".to_string(),
        " ".to_string(),
        "### Cloud Solutions ".to_string(),
        " ".to_string(),
        "- Why Rust is the most admired language among developers ".to_string(),
        " ".to_string(),
        "- Optimize areas of code that consume the most time or resources ".to_string(),
        " ".to_string(),
        "- Rust and Sustainability: Programming for a Greener Future ".to_string(),
        " ".to_string(),
        "- Announcing Windows 11 Insider Preview Build 25905 ".to_string(),
        " ".to_string(),
    ];

    worst_case_group.bench_function("Worst Case", |b| {
        b.iter(|| {
            let words_clone = words.clone();
            StringOps::calc_worst(black_box(words_clone))
        });
    });

    worst_case_group.finish();

    let mut green_case_group = c.benchmark_group("String Concatenate Green Case");
    green_case_group.measurement_time(Duration::from_secs(10));

    green_case_group.bench_function("Green Case", |b| {
        b.iter(|| {
            let words_clone = words.clone();
            StringOps::calc_green(black_box(words_clone))
        });
    });

    green_case_group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(50);
    targets = concatenate_benchmark
}
criterion_main!(benches);
