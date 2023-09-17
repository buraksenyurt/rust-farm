use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    //inital();
    //do_with_threads();
    do_with_handles();
}

fn do_with_handles() {
    /*
       İlk döngüdeki thread'in bitmesi beklenir.
       Böylece ana thread'in diğer thread bitmeden sonlanması yani programın kapanması engellenir.
    */

    let t_handle = thread::spawn(|| {
        for i in 0..10 {
            println!("İlk döngü. İş -> {i}");
            sleep(Duration::from_millis(1000));
        }
    });

    for i in 0..10 {
        println!("İkinci döngü. İş -> {i}");
        sleep(Duration::from_millis(500));
    }

    t_handle.join().unwrap();
}

fn do_with_threads() {
    /*
       İlk döngü ayrı bir thread içinde çalıştırılır.
       Problem şudur. İkinci döngü işleri birinci döngünün
       çalıştığı thread bitmeden biter.
       Main bu sebeple hemen sonlanır.
       Asnekron devam eden işlerin tamamlanmasını beklemek için handle nesneleri kullanılır.
    */
    thread::spawn(|| {
        for i in 0..10 {
            println!("İlk döngü. İş -> {i}");
            sleep(Duration::from_millis(1000));
        }
    });

    for i in 0..10 {
        println!("İkinci döngü. İş -> {i}");
        sleep(Duration::from_millis(500));
    }
}

fn inital() {
    // Başlangıç Konumu
    // Tüm işler senkron çalışır
    // Önce birinci döngünün bitmesi beklenir
    // ardından ikinci döngü işleri devam eder
    for i in 0..10 {
        println!("İlk döngü. İş -> {i}");
        sleep(Duration::from_millis(1000));
    }

    for i in 0..10 {
        println!("İkinci döngü. İş -> {i}");
        sleep(Duration::from_millis(500));
    }
}
