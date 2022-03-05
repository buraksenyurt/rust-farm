use std::io::{Read, Write};
use std::{env, io};

/*
   Program, terminal girdilerini 512 byte'lık bloklar haline okuyacak.
   Onu bir sabit olarak tanımlayabiliriz.
*/
const BLOCK_SIZE: usize = 512;

fn main() {
    // environment'ten bir parametre alıp nasıl kullandığımıza bakalım.
    let shadow_mode = env::var("SHADOW").unwrap_or_default() == "ON";
    // Toplamda kaç byte işlediğimizi görmek istiyoruz
    let mut total_bytes = 0;
    // Örnek sonsuz bir döngü içinde çalışıyor.
    // Yani kullanıcı kapatana kadar girdi alabilir.
    loop {
        // Gelen girdiyi hep 512 byte'lık bloklar halinde okumaktayız
        let mut buffer = [0; BLOCK_SIZE];
        // read çağrısı ile okunan içeriği buffer'a yazmaktayız.
        // Eğer okunacak bir içerik yoksa veya hata varsa döngüyü kırıyoruz.
        // Aksi durumda okunan içeriğin uzunluğunu yakalıyoruz.
        let bytes_read = match io::stdin().read(&mut buffer) {
            Ok(0) => {
                println!("\tİçerik sonlandı");
                break;
            }
            Ok(length) => length,
            Err(e) => {
                println!("Bir hata oluştu.{}", e);
                break;
            }
        };
        // Bir önceki adımda ne kadar byte okuduysak ekliyoruz
        total_bytes += bytes_read;
        // Okunan içeriği yazdığımız buffer'ı okunan byte kadar çıktıya iletiyoruz.
        // Bu çıktı duruma göre terminal ekranı, bir dosya veya ağ paketi de olabilir
        match io::stdout().write_all(&buffer[..bytes_read]) {
            Ok(_) => {}
            Err(e) => println!("Yazma işlemi sırasında hata. {}", e),
        };
    }

    // Environment değişkeninin nasıl kullandığımıza bakın.
    // Eğer SHADOW=ON şeklinde bir şey gelmişse total_bytes ekrana basılmayacak.
    if !shadow_mode {
        println!("Toplam {} byte veri işlendi.", total_bytes);
    }
}
