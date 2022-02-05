use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    /*
       # Senaryo 1

       Olabildiğince basit bir örnekle ilerleyelim.
       Main thread içinden iki thread başlatalım.
       Bu thread'ler bir transmitter kullanarak kanala çeşitli bilgiler bıraksınlar.
       Ana thread'de bir dinleyici olarak bu kanala gelen mesajları alsın.
    */

    let chrono = SystemTime::now();

    // channel fonksiyonu bir transmitter ve birde consumer kanalı oluşturur
    let (cmd_transmitter, cmd_receiver) = mpsc::channel();
    // İKinci bir transmitter nesnesini birincisinden klonlarız.
    // Böylece ikinci thread aynı kanala mesaj bırakabilir.
    let cmd_transmitter2 = cmd_transmitter.clone();

    // İki thread açacağız. Bu thread'ler sonlanmadan main bitsin istemeyiz.
    let mut handlers = vec![];

    // bir thread açıyoruz ve cmd_transmitter ile işlem sonunda kanala mesaj bırakıyoruz.
    handlers.push(thread::spawn(move || {
        println!("#{:?} Yolcu#23 sefere başlıyor.", thread::current().id());
        sleep(Duration::from_secs(3));
        cmd_transmitter.send("Yolcu#23 hedefte.").unwrap();
    }));

    // Burada ikinci bir thread söz konusu ve bu thread işini bitirdiğinde ilk transmitter
    // clone'u üstünden yine kanala bir mesaj bırakıyor.
    handlers.push(thread::spawn(move || {
        println!(
            "#{:?} Kaşif#24 warp hızlanma motoru aktif.",
            thread::current().id()
        );
        sleep(Duration::from_secs(5));
        cmd_transmitter2.send("Kaşif#24 öte evrene ulaştı").unwrap();
    }));

    // Başlatılan thread'ler bittikçe kanala bıraktıkları mesajları okuyoruz.
    for h in handlers {
        let _ = h.join();
        let msg = cmd_receiver.recv().unwrap();
        println!("İşlem bilgisi : {}", msg);
    }

    println!(
        "İşlemler {} saniyede tamamlandı",
        chrono.elapsed().unwrap().as_secs_f32()
    );
}
