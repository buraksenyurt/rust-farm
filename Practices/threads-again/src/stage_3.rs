use std::str::FromStr;
use std::thread;

/*
   thread'leri spawn'ladığımızda olası hataları thread::Result ile değerlendirebiliriz.
   Örnekte Block nesnesini kullanan ve içerisinde bir thread başlatan fonksiyonda sembolik bir hata
   oluşuyor. Bu hata mesajı main'de Result dönüşüne göre pattern matching ile yakalanabilir.
   Ayrıca Block nesne örneği Drop sürecine girdiği noktada da
   o anki thread'in panikleyip paniklemediği anlaşılabilir.
*/
fn main() {
    let block = Block {
        source: "123A".to_string(),
    };
    match calculate(block) {
        Ok(_) => println!("Hesaplamalar yapıldı..."),
        Err(_) => println!("İşlemler hata aldığı için başarısız oldu!"),
    }
}

struct Block {
    source: String,
}

// Block nesnesi için destructor'u yeniden kodluyoruz.
impl Drop for Block {
    fn drop(&mut self) {
        // Block nesnesinin içine dahil olduğu thread'de bir panikleme var mı kontrol ediliyor
        if thread::panicking() {
            println!(
                "Drop sürecinde bir panik algılandı. {:?}",
                thread::current().id()
            );
        }
    }
}

// İçerisinde thread başlatan sembolik fonksiyon
fn calculate(block: Block) -> thread::Result<()> {
    // block isimli metot parametresinin sahipliğinin alt thread'e geçmesi gerekiyor.
    // Bunu sağlamak için closure'da move operatörü kullanılır.
    thread::spawn(move || {
        println!(
            "'{}' numaralı block için sıkıştırma işlemleri başlatıldı. {:?}",
            &block.source,
            thread::current().id()
        );
        // main'den gönderdiğimiz string değer sayısala çevrilemeyeceği için bir hata oluşacak
        let _id = i32::from_str(&block.source).expect("Geçersiz block değeri");
    })
    .join() // çağıran ana thread'e dahil edilecek(örnekte main)
}
