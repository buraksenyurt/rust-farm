use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // localhost:3001 adresini dinleyecek şekilde bir nesne tanımlanır
    let listener = TcpListener::bind("127.0.0.1:3001").unwrap();
    println!("Sunucu dinliyor... localhost:3001");
    // Birden fazla istemci bağlanacağını düşündüğümüzden
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // her birin i ayrı ayrı ele alacak şekilde stream'leri birer thread'e açıyoruz.
                thread::spawn(move || {
                    handler(stream);
                });
            }
            Err(e) => {
                println!("Bir sorun var! {}", e);
            }
        }
    }
}

// stream'leri ele ala iş parçacığı
fn handler(mut stream: TcpStream) {
    // 32 byte'lık içerik okuyacağız
    let mut buffer = [0; 32];
    loop {
        match stream.read(&mut buffer) {
            // Okunacak byte kalmayan kadar okuyoruz
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                // İstemciden gelen mesajı ekrana basıyoruz
                println!(
                    "From Client:\t{}",
                    String::from_utf8_lossy(&buffer[..bytes_read])
                );
                // Echo amaçlı olarak istemciden gelen mesajı kendisine yolluyoruz.
                stream.write_all(&buffer[..bytes_read]).unwrap();
            }
            Err(_) => break,
        }
    }
}
