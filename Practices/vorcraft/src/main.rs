use crossbeam::channel;
use std::thread;
use vorcraft::{pesant_worker, Job};

fn main() {
    env_logger::init();

    println!("Oyun başladı. Görevler dağıtılacak.");

    // İlk olarak unbounded kanallarımızı oluşturalım.
    // unbounded bir Tuple döner.
    // jt -> Jobs Transmitter, jr -> Jobs Receiver anlamında.
    // rt -> Results Transmitter, rr -> Results Receiver anlamında.
    let (jt, jr) = channel::unbounded();
    let (rt, rr) = channel::unbounded();

    let jr2 = jr.clone();
    let rt2 = rt.clone();
    let jr3 = jr.clone();
    let rt3 = rt.clone();

    // Şimdi üç thread oluşturacağız. Bunları JoinHandle serisinde toplayabiliriz.
    // Tohumlanan thread'ler pesant_worker fonksiyonunu çağırmakta ve buraya birer reciver ile
    // transmitter nesnesi göndermekte. Ancak her thread kendi transmitter ve receiver'ı ile çalışmalı.
    // Bu nedenle bir üst satırda clone'landıklarını görebiliriz.
    let handles = vec![
        thread::spawn(|| pesant_worker(jr, rt)),
        thread::spawn(|| pesant_worker(jr2, rt2)),
        thread::spawn(|| pesant_worker(jr3, rt3)),
    ];

    // Birkaç kobay iş isteiğinden oluşan bir vector hazırlayalım
    let jobs = vec![
        Job::WheatFarm,
        Job::FishFarm,
        Job::Shack(8),
        Job::Ditch(23.0),
        Job::ArcherTower(100),
        Job::Shack(4),
        Job::FishFarm,
        Job::ArcherTower(50),
        Job::Shack(10),
    ];

    // Herbir iş isteğini ilgili kanala bırakacak bir döngü.
    for j in jobs {
        println!("İstenen iş {:?}", j);
        let _ = jt.send(j); // Kanala istenen işi bıraktık
    }
    drop(jt);

    // Burada da thread'lerin yaptığı iş sonuçlarının aktığı kanalı dinleyeceğiz
    for r in rr {
        println!("Tamamlanan iş {:?}", r); // ve şimdi de çağırılan thread'den kanala bırakan sonuç mesajını aldık
    }

    // İşlemler bitmeden main'in sonlanmasını engelleyelim
    for h in handles {
        let _ = h.join();
    }
}
