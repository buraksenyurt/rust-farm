use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Birkaç thread'in ortaklaşa kullanmak istediği bir sayı var.
    // Thread Safe Reference Counter ile birlikte Mutex nesnesi olarak oluşturuyoruz
    let popular_number = Arc::new(Mutex::new(23_u32));

    // JoinHandle nesneleri için bir vector. Main'i bekletmek için kullanırız.
    let mut handlers = Vec::new();

    // 10 tane thread açalım
    for _ in 0..10 {
        // Atomic Reference Counter'ın bir klonunu oluşturuyoruz
        // Thread'ler bunu kullanacak
        let number_clone = Arc::clone(&popular_number);

        handlers.push(thread::spawn(move || {
            // klonlanan referans bilgisine bağlı Mutex içeriğini kitliyoruz
            let mut p = number_clone.lock().unwrap();

            println!("{:?} Parametre : {:?}", thread::current().id(), p);

            // p değerini dereference ettikten sonra 1 artırıyoruz
            *p += 1;

            println!("\t{:?} Yeni Değer : {:?}", thread::current().id(), p);
        }));
    }

    for h in handlers {
        let _ = h.join();
    }

    // Thread'lerin ortaklaşa kullandığı sayısal değer aslında Atomic Reference Counter tarafından
    // tutulan bir Mutext değişkeni. Bir şekilde sonucu almak gerekiyor.
    // Şimdilik aşağıdaki gibi bir yol bulabildim.
    println!(
        "Sayının değeri {}",
        Arc::try_unwrap(popular_number)
            .unwrap()
            .into_inner()
            .unwrap()
    );
}
