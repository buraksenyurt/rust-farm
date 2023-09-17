use std::f32::consts::{E, PI};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    //inital();
    //do_with_threads();
    //do_with_handles();
    // take_ownership_err();
    // take_ownership();
    // use_mpsc();
    //use_sender_on_function();
    //send_multi_values();
    //send_multi_values_with_loop();
    //send_with_cloned_transmitter();
    handle_data_races();
}

fn handle_data_races() {
    /*
       010
       aynı veriyi kullanmak isteyen thread'ler söz konusu olduğunda data races durumunu
       engellemek için Arc ve Mutex enstrümanlarından yararlanılabilir.
       shared_value hem thread-safe dir hem de lock'lanabilir
    */
    let shared_value = Arc::new(Mutex::new(PI));
    let mut handles = vec![];
    for i in 0..5 {
        let safe_value = Arc::clone(&shared_value);
        let handle = spawn(move || {
            let mut value = safe_value.lock().unwrap();
            *value += 1.;
            println!("Güncel değer {}", value);
            sleep(Duration::from_millis(500));
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Sonuç {}", *shared_value.lock().unwrap());
}

fn send_with_cloned_transmitter() {
    /*
       009
       Kanallara n sayıda mesajı n sayıda thread açarak da gönderebiliriz.
       Bunun için transmitter'ı klonlamak yeterlidir.
    */
    let (transmitter_1, receiver) = channel();
    let transmitter_2 = Sender::clone(&transmitter_1);
    let transmitter_3 = Sender::clone(&transmitter_1);
    spawn(move || {
        transmitter_1.send("Oyuncu eklensin").unwrap();
        sleep(Duration::from_millis(100));
    });
    spawn(move || {
        transmitter_2.send("Rakip eklensin").unwrap();
        sleep(Duration::from_millis(50));
    });
    spawn(move || {
        transmitter_3.send("Çalılar eklensin").unwrap();
        sleep(Duration::from_millis(200));
    });
    for r in receiver {
        println!("Log:{r}");
    }
}

fn send_multi_values_with_loop() {
    /*
       008
       Kanallara n sayıda mesaj gönderimi de mümkündür.
       Bunun için bir vektor kullanılabileceği gibi send fonksiyonu ardışıl olarak
       n defa da çağrılabilir.
    */
    let (transmitter, receiver) = channel();
    let handle = spawn(move || {
        let variables = vec![1, 3, 5, 7, 9, 11];
        for v in variables {
            transmitter.send(v).unwrap();
            sleep(Duration::from_millis(1000));
        }
    });
    handle.join().unwrap();
    for recv in receiver {
        let _ = spawn(move || {
            println!("Gelen değer {recv}");
            sleep(Duration::from_millis(250));
        });
    }
}

fn send_multi_values() {
    /*
       007
       Kanallara n sayıda mesaj gönderimi de mümkündür.
       Bunun için bir vektor kullanılabileceği gibi send fonksiyonu ardışıl olarak
       n defa da çağrılabilir.
    */
    let (transmitter, receiver) = channel();
    let handle = spawn(move || {
        let variables = vec![1, 3, 5, 7, 9, 11];
        transmitter.send(variables).unwrap();
        sleep(Duration::from_millis(1000));
    });
    handle.join().unwrap();
    let handle = spawn(move || {
        process_variables(receiver);
    });
    handle.join().unwrap();
}
fn process_variables(receiver: Receiver<Vec<i32>>) {
    let values = receiver.recv().unwrap();
    for v in values.iter() {
        println!("Gelen değer {v}");
        sleep(Duration::from_millis(250));
    }
}

fn use_sender_on_function() {
    /*
       006
       Bu senaryoda transmitter ve receiver nesneleri fonksiyonlara
       parametre olarak geçilerek kullanılmaktadır.
    */
    let (transmitter, receiver) = channel();
    let handle = spawn(move || {
        write_log(
            transmitter,
            "Servis istek sayısı limit üstüne çıktı.".to_string(),
        );
    });
    handle.join().unwrap();

    let handle = spawn(move || {
        read_log(receiver);
    });
    handle.join().unwrap();
}

fn write_log(sender: Sender<String>, message: String) {
    sender.send(message).unwrap();
}
fn read_log(receiver: Receiver<String>) {
    let message = receiver.recv().unwrap();
    println!("Gelen log mesajı: '{message}'");
}

fn use_mpsc() {
    /*
       005
       thread'lar arası mesaj taşımanın br yolu kanalları(channels) kullanmaktır.
       varsayılan olarak Multiple Producer Single Consumer ilkesine göre çalışır.
       transmitter mesaj gönderir, receiver mesaj yakalar.
    */
    let (transmitter, receiver) = mpsc::channel();
    let _ = spawn(move || {
        let calculated = PI * E * 5.;
        transmitter.send(calculated).unwrap();
    });

    let catched = receiver.recv().unwrap();
    println!("Catched value {catched}");
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
    handle.join().unwrap();
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
