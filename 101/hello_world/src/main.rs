/*
    Hello World

    Program terminalden girdi alır ekrana basar.

    Çalıştırılabilir programların giriş noktası main fonksiyonudur.
    Rust dilinde tüm değişkenler varsayılan olarak Immutable'dır. mut ile değiştirilebilir olurlar.
    match ifadesi ile bir değerin tüm olası koşulları kontrol edilebilir.

 */

// io kütüphanesini bildirimi
use std::io;

fn main() {
    // Yeni bir String nesnesi oluşturulur
    let mut user_name: String = String::new();

    // Ekrana bir mesaj yazar
    println!("Merhaba. Bana adını söyler misin?");

    // pattern matching kullanımı
    // read_line fonksiyonu ile terminal girdisi input değişkenine alınır
    // read_line çıktısı Result<T,Error> tipindendir
    // match ile Ok ve Err halleri kontrol edilir
    match io::stdin().read_line(&mut user_name) {
        Ok(_) => println!("Merhaba, {}Rust çiftliğine hoş geldin.", user_name.to_uppercase()),
        Err(e) => println!("Bir şeyler ters gitmiş olabilir. {}", e),
    };
}
