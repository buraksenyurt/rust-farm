use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    /*
       Standart bir dosya içeriği okuma işlemi. byte array olarak.
    */

    // Dosya açma
    let mut f = File::open("data.txt").unwrap();
    // İçeriğin aktarılacağı bir byte array
    let mut buffer = [0; 1024];
    // Dosya içeriği byte array'e alma
    let _ = f.read(&mut buffer[..]).unwrap();
    // Çıktı bir byte array olarak ekrana yazdırılır.
    println!("{:?}", buffer);

    /*
        Dosya içeriğini BufReader ile okumak(Komple okuma)
    */
    // Yine okumak istenen dosya için bir nesne örneklenir
    let f = File::open("data.txt").unwrap();
    // Bu sefer okuyucu BufReader
    let mut buf_reader = BufReader::new(f);
    // İçerik bir String'e aktarılacak
    let mut buffer = String::new();
    // BufReader ile dosya içeriği buffer isimli String'e alınır
    buf_reader.read_to_string(&mut buffer).unwrap();
    println!("\n{}", buffer);

    /*
        Dosya içeriğini BufReader ile okumak(Satır bazlı okuma)
    */
    // Yine okumak istenen dosya için bir nesne örneklenir
    let f = File::open("data.txt").unwrap();
    // Bu sefer okuyucu BufReader'ın lines fonksiyonu ile dosya içeriği
    // satır bazlı yakalanır.
    let buf_reader = BufReader::new(f).lines();
    for line in buf_reader {
        println!("{}", line.unwrap());
    }
}
