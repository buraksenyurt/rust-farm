/*
   Tam anlamıyla istenilen şekilde asenkron çalışan bir web server yazmak sanıldığı kadar
   kolay değildir. Bu nedenle harici bir runtime kullanmak çok daha mantıklı.
   tokio bu aşamadan ilk tercih edilen crate gibi görünüyor.
*/
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::signal::ctrl_c;
use tokio::sync::Notify;

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("localhost:4000")
        .await
        .expect("4000 portu açılamadı");
    let state = Arc::new((AtomicUsize::new(0), Notify::new()));
    println!("localhost:4000");

    loop {
        select! {
            result = listener.accept() => {
                let (connection, _) = result.unwrap();
                let state = state.clone();

                state.0.fetch_add(1, Ordering::Relaxed);

                tokio::spawn(async move {
                    if let Err(e) = HttpModule::handle_request(connection).await {
                        println!("{e}")
                    }

                    let count = state.0.fetch_sub(1, Ordering::Relaxed);
                    if count == 1 {
                        state.1.notify_one();
                    }
                });
            }
            _shutdown = ctrl_c() => {
                let timer = tokio::time::sleep(Duration::from_secs(15));
                let request_counter = state.1.notified();

                if state.0.load(Ordering::Relaxed) != 0 {
                    select! {
                        _ = timer => {}
                        _ = request_counter => {}
                    }
                }

                println!("Kapatılıyor...");
                return;
            }
        }
    }
}

struct HttpModule;

impl HttpModule {
    // bağlantı üstünden gelen stream'i ele aldığımız fonksiyon
    async fn handle_request(mut conn: TcpStream) -> io::Result<()> {
        let mut read = 0;
        let mut buffer = [0u8; 1024]; //1024er byte oku

        loop {
            // belirtilen read noktasından itibaren 1024 byte bilgiyi buffer'a al
            let read_bytes = conn.read(&mut buffer[read..]).await?;

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
            let write_bytes = conn.write(dummy_response[written..].as_bytes()).await?;

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

        conn.flush().await
    }
}
