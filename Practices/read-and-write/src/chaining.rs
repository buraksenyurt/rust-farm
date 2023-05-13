use std::fs::File;
use std::io::Read;

fn main() {
    /*
       io modülü okumalarda zincir oluşturabilmek için chain fonksiyonunu sunar.
       Örneğin n sayıda dosyanın birleştirilmesinde kullanılabilir.
    */

    // İki kaynak dosya için birer nesne oluşturulur
    if let Ok(file_1) = File::open("list_1.txt") {
        if let Ok(file_2) = File::open("list_2.txt") {
            // Dosyalar birbirine zincirlenir
            let mut chain_handler = file_1.chain(file_2);
            let mut buffer = String::new();
            // Zincirlenmiş dosya içeriği tek blokta buffer içersine okunur
            let content = chain_handler.read_to_string(&mut buffer);
            match content {
                Ok(_) => println!("Ürün Listesi\n{}", buffer),
                Err(e) => println!("Dosya okumada hata {}", e),
            }
        }
    }
}
