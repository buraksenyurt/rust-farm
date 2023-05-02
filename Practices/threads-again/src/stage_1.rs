use std::thread;

fn main() {
    /*
       Bu örnekte standart library'teki spawn fonksiyonu kullanılarak 3 thread açılmakta.
       Main thread'in bu thread'ler sonlanmadan kapanmaması için de açılan thread'i bir vector
       dizisi ile kontrol etmekteyiz.
    */
    let mut threads = Vec::new();

    for _ in 0..7 {
        let t_handle = thread::spawn(|| {
            println!("{:?} nolu bir thread açıldı", thread::current().id());
        });
        threads.push(t_handle);
    }
    // Tüm thread'ler ana thread'e katılır
    for th in threads {
        th.join().unwrap();
    }
}
