use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Arc (Atomically Referenced Counted Type) Verinin primitive tiplere indirgenerek
    // thread'ler arasında güvenli şekilde paylaşılmasını sağlar.

    // Thread'lerin ortaklaşa kullanacaığı bir counter tanımladık
    let vote_counter = Arc::new(Mutex::new(0));
    // thread'leri toplayacağımı vecktor
    let mut threads = vec![];

    // sembolik olarak 10 thread başlatacağız
    for _ in 0..10 {
        // İlk olarak Arc nesnesinin bir klonunu oluşturmalıyız
        let klu = Arc::clone(&vote_counter);
        // thread açıldığında dikkat edileceği üzere
        threads.push(thread::spawn(move || {
            println!("#{:?} oy veriyor", thread::current().id());
            // Minik bir bekletme yapalım
            sleep(Duration::from_secs_f32(1.75));
            // değişkeni kilit altına alırken değerini bir artırıyoruz
            let mut number = klu.lock().unwrap();
            // Başlatılan her thread number referansını dolayısıyla vote_counter değerini bir artırır
            *number += 1;
            println!("\t#{:?} oyladı. Oy değeri {}", thread::current().id(),number);
        }));
    }

    // Main'in tüm thread'lerin bitmesini beklemesi için join'den yararlanabiliriz
    for t in threads {
        let _ = t.join();
    }

    // Arc değerini ekrana yazdırıyoruz. Yine lock kullandığımıza dikkat edelim.
    println!(
        "Rust'ın çok güzel bir dil olduğunu kabul edenler {}",
        *vote_counter.lock().unwrap()
    );
}
