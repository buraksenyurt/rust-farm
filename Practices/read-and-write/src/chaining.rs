use std::fs::File;
use std::io::Read;

fn main() {
    /*
       io modülü okumalarda zincir oluşturabilmek için chain fonksiyonunu sunar.
       Örneğin n sayıda dosyanın birleştirilmesinde kullanılabilir.
    */

    // İki kaynak dosya için birer nesne oluşturulur
    let list_1 = File::open("list_1.txt").unwrap();
    let list_2 = File::open("list_2.txt").unwrap();

    // Dosyalar birbirine zincirlenir
    let mut chain_handler = list_1.chain(list_2);
    let mut buffer = String::new();
    // Zincirlenmiş dosya içeriği tek blokta buffer içersine okunur
    chain_handler.read_to_string(&mut buffer).unwrap();
    println!("Ürün Listesi\n{}", buffer);
}
