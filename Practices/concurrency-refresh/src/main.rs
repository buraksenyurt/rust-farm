use std::thread::sleep;
use std::time::Duration;

fn main() {
    inital();
}

fn inital() {
    // Başlangıç Konumu
    // Tüm işler senkron çalışır
    for i in 0..10 {
        println!("İlk döngü. İş -> {i}");
        sleep(Duration::from_millis(1000));
    }

    for i in 0..10 {
        println!("İkinci döngü. İş -> {i}");
        sleep(Duration::from_millis(500));
    }
}
