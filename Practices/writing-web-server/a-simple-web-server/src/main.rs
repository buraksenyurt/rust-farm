/*
   En İlkel haliyle bir web server

   Dezavantaj;

   Bu modelin problemi, sunucunun t anında sadece tek bir request ele alması.
   Aynı anda n sayıda kullanıcının olduğu senaryolarda sunucuya t anından n sayıda tale gelecektir.
*/
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("localhost:4000").expect("4000 nolu port bağlanamadı");
    println!("Sunucu -> localhost:4000");
    loop {
        let (conn, _) = listener.accept().expect("Talepler kabul edilemiyor");

        if let Err(e) = HttpModule::handle_request(conn) {
            println!("Problem var. Request okunamıyor. Detay {e}");
        }
    }
}

struct HttpModule;

impl HttpModule {
    // bağlantı üstünden gelen stream'i ele aldığımız fonksiyon
    fn handle_request(mut conn: TcpStream) -> io::Result<()> {
        let mut read = 0;
        let mut buffer = [0u8; 1024]; //1024er byte oku

        loop {
            // belirtilen read noktasından itibaren 1024 byte bilgiyi buffer'a al
            let read_bytes = conn.read(&mut buffer[read..])?;

            if read_bytes == 0 {
                return Ok(()); // istemci bağlantısı beklenmedik şekilde kesildiyse
            }

            read += read_bytes;

            // buffer'a alınan verinin sonuna gelip gelmediğimizi anlamak için
            if buffer.get(read - 4..read) == Some(b"\r\n\r\n") {
                break;
            }
        }

        // Request ile gelen mesajı ekrana basmak için
        let request = String::from_utf8_lossy(&buffer[..read]);
        println!("->{request}");

        let dummy_response = concat!(
            "HTTP/1.1 200 OK\r\n",
            "Content-Length: 17\n",
            "Connection: close\r\n\r\n",
            "EnCinX Server 1.0"
        );

        let mut written = 0;
        loop {
            let write_bytes = conn.write(dummy_response[written..].as_bytes())?;

            if write_bytes == 0 {
                println!("istemci bekenmedik şekilde bağlantıyı kesti.");
                return Ok(());
            }

            written += write_bytes;

            // tüm mesaj yazıldıysa döngüden çık
            if written == dummy_response.len() {
                break;
            }
        }

        conn.flush()
    }
}
