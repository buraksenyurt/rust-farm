use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    /*
    Çok kolay bir şekilde n adet thread'i aşağıdaki şekilde başlatabiliriz.
    Ortamda main ile birlikte 4 thread açılacaktır.

    Blok içerisinde i değerine göre belli bir süre duraksatma işlemi uygulanır.
    Tamemen uzun süren işleri simüle etmek için eklenmiştir.

    Main thread'in diğer thread'lerin işini bitirmesini beklemesi tercih edilir.
    Bu nedenle açılan thread'ler bir vector'de toplanmaktadır ve join ile sonlanmaları beklenmektedir.
     */
    let mut threads = vec![];

    let now = SystemTime::now();

    for i in 1..=4 {
        /*
           for döngüsündeki i değişkenini thread içerisine alabilmek için move kullanılır.
           Bunu yapmassak şu uyarıyı alırız;

           "to force the closure to take ownership of `i` (and any other referenced variables), use the `move` keyword"

           thread'ler move ile içeriye değer alınmasını sağlarken değer de döndürebilirler.
           Aşağıdaki döngüler iş bittikten sonra i değerini geriye döndürür.
        */
        let t = thread::spawn(move || {
            println!(
                "#{:?} nolu iş başlatıldı. Gelen değer {}",
                thread::current().id(),
                i
            );
            sleep(Duration::from_secs_f32(i as f32 * 1.10));
            i * 10
        });
        threads.push(t);
    }

    for thread in threads {
        let result = thread.join();
        println!("Sonuç {}", result.unwrap());
    }
    println!(
        "İşler tamamlandı. Geçen toplam süre {:?}",
        now.elapsed().unwrap()
    );
}
