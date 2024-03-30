use bad_or_good::Sorting;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::time::Duration;

/*
   Bu örnekte Bubble Sort'un Radix Sort'tan hızlı çalışıp çalışamayacağı konusu araştırılıyor.

   Teoriye göre neredeyse sıralanmış bir veri setinden Bubble Sort, Radix'e göre daha iyi sonuç
   verebilir. Bu nedenle create_data_set metodu içerisinde sıralanmış bir veri seti üretilip,
   sonrada bu verisetine sıralanmamış birkaç eleman ekleniyor.
*/

fn create_data_set(size: usize, max: i32, unsorted_element_count: usize) -> Vec<i32> {
    let mut data: Vec<i32> = (0..size).map(|x| x as i32 % max).collect();
    data.sort();
    let mut rng = rand::thread_rng();

    for _ in 0..unsorted_element_count {
        let index1 = rng.gen_range(0..size);
        let index2 = rng.gen_range(0..size);
        data.swap(index1, index2);
    }

    data
}

fn maybe_bubble_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Benchmarks");
    for size in [10, 10_000, 50_000].iter() {
        let max_value = 10_000;
        let data = create_data_set(*size, max_value, 5);

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
    config = Criterion::default().sample_size(10);//.measurement_time(Duration::from_secs(30));
    targets = maybe_bubble_benchmarks
}
criterion_main!(benches);
