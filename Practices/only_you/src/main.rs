//use std::sync::{Arc, Mutex};
//use std::thread;

use parking_lot::Mutex;

fn main() {
    // Yine bir Mutex oluşturduk ancak Arc gibi bir referans counter kullanmadık
    let popular_number = Mutex::new(23_u32);

    // Bir scope oluşturuyoruz ve bu scope 10 adet thread yumurtluyor.
    crossbeam::scope(|s| {
        for _ in 0..10 {
            s.spawn(|_| {
                // Dereference ve lock işlemini mütaakip sayı değerini bir artırıyoruz
                *popular_number.lock() += 1;
            });
        }
    })
    .unwrap();

    let popular_number = popular_number.into_inner();
    println!("Sayının değeri {}", popular_number);

    // // Birkaç thread'in ortaklaşa kullanmak istediği bir sayı var.
    // // Thread Safe Reference Counter ile birlikte Mutex nesnesi olarak oluşturuyoruz
    // let popular_number = Arc::new(Mutex::new(23_u32));
    //
    // // JoinHandle nesneleri için bir vector. Main'i bekletmek için kullanırız.
    // let mut handlers = Vec::new();
    //
    // // 10 tane thread açalım
    // for _ in 0..10 {
    //     // Atomically Reference Counted'ın nesnesinin bir klonunu oluşturuyoruz
    //     // Thread'ler bunu kullanacak
    //     let number_clone = Arc::clone(&popular_number);
    //
    //     handlers.push(thread::spawn(move || {
    //         // klonlanan referans bilgisine bağlı Mutex içeriğini kitliyoruz
    //         let mut p = number_clone.lock().unwrap();
    //
    //         println!("{:?} Parametre : {:?}", thread::current().id(), p);
    //
    //         // p değerini dereference ettikten sonra 1 artırıyoruz
    //         *p += 1;
    //
    //         println!("\t{:?} Yeni Değer : {:?}", thread::current().id(), p);
    //     }));
    // }
    //
    // for h in handlers {
    //     let _ = h.join();
    // }
    //
    // // Thread'lerin ortaklaşa kullandığı sayısal değer aslında Atomically Reference Counted tarafından
    // // tutulan bir Mutext değişkeni. Bir şekilde sonucu almak gerekiyor.
    // // Şimdilik aşağıdaki gibi bir yol bulabildim.
    // println!(
    //     "Sayının değeri {}",
    //     Arc::try_unwrap(popular_number)
    //         .unwrap()
    //         .into_inner()
    //         .unwrap()
    // );
}
