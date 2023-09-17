use std::f32::consts::PI;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    //inital();
    //do_with_threads();
    //do_with_handles();
    // take_ownership_err();
    take_ownership();
}

fn take_ownership() {
    // 004
    // 003'teki soruna istinaden move keyword'ü ile sahipliğin thread'a alınması denenebilir
    let variable = PI;
    let handle = spawn(move || {
        for i in 0..10 {
            println!("{}", i as f32 * variable);
            sleep(Duration::from_millis(500));
        }
    });
    handle.join();
}

// fn take_ownership_err() {
//     // 003
//     // thread içine dışardaki bir değişkenin sahipliği alınmak istenmiştir
//     // ownership kurallarına göre bir değerin tek sahibi olabilir.
//     // error[E0373]: closure may outlive the current function, but it borrows `variable`, which is owned by the current function
//     // hatası oluşur
//     let variable = PI;
//     let handle = spawn(|| {
//         for i in 0..10 {
//             println!("{}", i as f32 * variable);
//         }
//     });
//     handle.join();
// }

fn do_with_handles() {
    /*
       002
       İlk döngüdeki thread'in bitmesi beklenir.
       Böylece ana thread'in diğer thread bitmeden sonlanması yani programın kapanması engellenir.
    */

    let t_handle = spawn(|| {
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
       001
       İlk döngü ayrı bir thread içinde çalıştırılır.
       Problem şudur. İkinci döngü işleri birinci döngünün
       çalıştığı thread bitmeden biter.
       Main bu sebeple hemen sonlanır.
       Asnekron devam eden işlerin tamamlanmasını beklemek için handle nesneleri kullanılır.
    */
    spawn(|| {
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
    // Başlangıç Konumu 000
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
