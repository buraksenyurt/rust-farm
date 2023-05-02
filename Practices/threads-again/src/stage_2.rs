use std::thread;

fn main() {
    /*
       Standart library kullanılarak thread oluşturmanın bir diğer yolu da,
       Builder'dan yararlanmak(Thread Factory olarak düşünülebilir)
       Builder ile thread'lere isim verebilir ve bellekteki stack alanı boyutlarını değiştirebiliriz.
    */
    let mut threads = Vec::new();

    for i in 0..7 {
        let t_builder = thread::Builder::new()
            .name(format!("tiny_thread_{}", i))
            .stack_size(32 * 1024);
        let t_handle = t_builder
            .spawn(|| {
                println!(
                    "{:?} id'li {:?} thread.",
                    thread::current().id(),
                    thread::current().name()
                );
            })
            .expect("Thread oluşturulamadı");
        threads.push(t_handle);
    }
    // Tüm thread'ler ana thread'e katılır
    for th in threads {
        th.join().unwrap();
    }
}
