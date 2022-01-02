/*
   Hello World

   Program terminalden girdi alır ekrana basar.

   Çalıştırılabilir programların giriş noktası main fonksiyonudur.
   Rust dilinde tüm değişkenler varsayılan olarak Immutable'dır. mut ile değiştirilebilir olurlar.
   match ifadesi ile bir değerin tüm olası koşulları kontrol edilebilir.
    println!, en sık kullanılan macro fonksiyonlarındandır.
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
        Ok(_) => println!(
            "Merhaba, {}Rust çiftliğine hoş geldin.",
            user_name.to_uppercase()
        ),
        Err(e) => println!("Bir şeyler ters gitmiş olabilir. {}", e),
    };

    // println! macro için başka kullanımlar

    // {} ile metin içerisine argüman yerleştirme
    println!("Ben {} ve {} yaşındayım.", "Burak Selim", 45);
    // argümanda matematiksel işlem kullanımı
    println!("3.14 * 10 * 10 = {}", 3.14 * 10.0 * 10.0);
    // {} argümanını sayılar ile işaretleyip metin içinde farklı konumlarda kullanabilir, tekrar ettirebiliriz.
    println!(
        "{0}, {2} elektronik cihaza sahip.\n Bir tanesi {3}. {0}'ın ayrıca {1}'ı da var.",
        "Burak", "VR", 2, "Desktop PC"
    );
    // Hexadecimal, binary ve octal gösterimler
    println!(
        "314 sayısının Hex değeri {:x}\nBinary değeri {:b}\nOctal sayı sistemindeki değeri {:o}",
        314, 314, 314
    );
    // Bazen argümanın metinsel gösterimi için çalışma zamanının varsayılan davranışı kullanılabilir (Debug trait kullanımı)
    println!("Points: {:?}", [3, 5, 7, 9, 11]);

    // metinsel ifadedeki argümanlara isim verilebilir
    println!(
        "İlk adım {first_name}.Soyadım {second_name}",
        first_name = "Burak Selim",
        second_name = "Senyurt"
    );
}
