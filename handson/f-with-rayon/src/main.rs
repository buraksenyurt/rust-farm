use rayon::current_num_threads;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    print_thread_count();
    // find_total_scenario();
    find_primes_scenario();
}

fn print_thread_count() {
    let num_threads = current_num_threads();
    println!("Rayon bu sistemde {} iş parçacığı kullanacak.", num_threads);
}

fn find_total_scenario() {
    let numbers: Vec<u64> = (1..=1_000_000).collect();

    let start = Instant::now();
    let serial_sum: u64 = numbers.iter().sum();
    let serial_duration = start.elapsed();
    println!("Seri toplama süresi: {:?}", serial_duration);

    let start = Instant::now();
    let parallel_sum: u64 = numbers.par_iter().sum();
    let parallel_duration = start.elapsed();
    println!("Paralel toplama süresi: {:?}", parallel_duration);

    assert_eq!(serial_sum, parallel_sum);
}

// Sayının asal olup olmadığını kontrol eden fonksiyon
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn parallel_find_primes(numbers: Vec<u64>, num_threads: usize) -> Vec<u64> {
    let chunk_size = numbers.len() / num_threads;
    let numbers = Arc::new(numbers);
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for i in 0..num_threads {
        let numbers = Arc::clone(&numbers);
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end = if i == num_threads - 1 {
                numbers.len()
            } else {
                start + chunk_size
            };

            let local_primes: Vec<u64> = numbers[start..end]
                .iter()
                .cloned()
                .filter(|&n| is_prime(n))
                .collect();

            let mut results = results.lock().unwrap();
            results.extend(local_primes);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    results
}

/*
   Kendi sistemimde seri hesaplama yani tek thread ile asal sayı bulma operasyonu
   4 dakika 41 saniye kadar sürdü.

   Thread'leri kendimiz manuel açarsak bu süre


   Rayon kullananılan durumda 12 çekirdekli sistem bunu yaklaşık olarak
   1 dakika 36 saniyede gerçekleştirdi.
*/
fn find_primes_scenario() {
    let numbers: Vec<u64> = (1..=100_000_000).collect();

    let start = Instant::now();
    let s_primes: Vec<u64> = numbers.iter().filter(|&&n| is_prime(n)).cloned().collect();
    let serial_duration = start.elapsed();
    println!("Seri asal sayılar hesaplama süresi: {:?}", serial_duration);

    let start = Instant::now();
    let p_primes = parallel_find_primes(numbers.clone(), 12);
    let parallel_duration = start.elapsed();
    println!(
        "Thread'ler ile paralel asal sayılar hesaplama süresi: {:?}",
        parallel_duration
    );
    assert_eq!(s_primes, p_primes);

    let start = Instant::now();
    let p_primes: Vec<u64> = numbers
        .par_iter()
        .filter(|&&n| is_prime(n))
        .cloned()
        .collect();
    let parallel_duration = start.elapsed();
    println!(
        "Paralel asal sayılar hesaplama süresi: {:?}",
        parallel_duration
    );

    assert_eq!(s_primes, p_primes);
    println!("Sonuçlar eşleşiyor! Seri ve paralel hesaplamalar aynı.");
}