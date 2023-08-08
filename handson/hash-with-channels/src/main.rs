use sha2::{Digest, Sha512};
use std::time::Instant;

fn main() {
    // #00
    /*
        Diyelim ki büyük bir veri setindeki elemanların Sha512 deki hash değerlerini ürettirmek istiyoruz.
        Bu örnekte sadece i64 türünden değerler kullanılıyor.

        İlk yol aşağıdaki gibi bir döngüyle tüm elemanları dolaşıp hash değerlerini üretmektir.

        Örneğin Release modda çalıştırmak daha anlamlı. Bu nedenle önce
        
        cargo build --release

        komutu ile target/release klasörüne alınan çıkıtıyı üretmek gerekiyor.
    */
    let start_time = Instant::now();

    for i in 0..1_000_000_000 {
        let pre_image = (i as u64).to_le_bytes();
        Sha512::digest(&pre_image);
    }

    println!("Toplam hesaplama süresi {:?}", start_time.elapsed());
}
