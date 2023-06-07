use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // localhost:3001 adresini dinleyecek şekilde bir nesne tanımlanır
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    println!("Sunucu dinliyor... localhost:3001");
    for stream in listener.incoming() {
        // Gelen stream'ler karşılama üzere mutable bir nesne oluşturuluyor
        let mut stream = stream.unwrap();
        // 32 byte'lık bloklar haline okuyabiliriz.
        let mut buffer = [0; 32];
        // gelen içeriği buffer'a alıyoruz
        stream.read(&mut buffer).unwrap();
        // Buffer içeriğini ekrana basıyoruz
        println!("From Client:\t{}", String::from_utf8_lossy(&buffer));
        stream.write(&mut buffer).unwrap();
    }
}
