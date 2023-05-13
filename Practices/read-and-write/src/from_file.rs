use std::fs::File;
use std::io::Read;

fn main() {
    /*
        Standart bir dosya içeriği okuma işlemi.
     */

    // Dosya açma
    let mut f = File::open("data.txt").unwrap();
    // İçeriğin aktarılacağı bir byte array
    let mut buffer = [0;1024];
    // Dosya içeriği byte array'e alma
    let _ = f.read(&mut buffer[..]).unwrap();
    // Çıktı bir byte array olarak ekrana yazdırılır.
    println!("{:?}",buffer);
}