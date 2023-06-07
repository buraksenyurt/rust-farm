use std::io::{stdin, Read, Write};
use std::net::TcpStream;

fn main() {
    // Kullanıcdan sürekli girdi almak için sonsuz bir döngümüz var.
    loop {
        // Sunucuya bağlanıyoruz
        match TcpStream::connect("localhost:3001") {
            // Mesaj yollanabilecek bir stream varsa devam ediyor
            Ok(mut stream) => {
                println!("Sunucuya gönderilmek üzere bir mesaj yazın.\nÇıkmak için CTRL+C.");
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Girdi okuma hatası");
                // Stream'a terminalden gelen girdiyi basıyoruz
                stream.write(input.trim().as_bytes()).unwrap();
                // Bu kısımda da sunucudan gelen cevabı ele alıyoruz
                let mut buffer = [0; 32];
                // Sunucudan dönen içeriği buffer'a alıyoruz
                stream.read(&mut buffer).unwrap();
                // ve ekrana basıyoruz.
                println!(
                    "Echo from server:\t {:?}",
                    std::str::from_utf8(&buffer)
                        .unwrap()
                        .trim_end_matches(char::from(0))
                );
            }
            Err(e) => {
                println!("Bağlantı hatası: {}", e);
                continue;
            }
        };
    }
}
