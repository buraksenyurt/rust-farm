/*
    a-non-blocking-server

    Burada çağıran thread asla bloklanmaz(Non-Block).
    İstemci bağlantıları birbirlerini bloklamayacak şekilde ele alınır.
    Dolayısıyla sunucu birden fazla request'i eş zamanlı(concurrently) olarak tek bir
    thread içerisinde ele alabilir. Eğer süreç işleyişinde operasyonlardan birisi bir sebeple blok_
    lanırsa, o anki state(Write,Read,Flush) saklanır ve bir sonraki adım işletilir.

    Dezavantaj;

    Aktif olan her tekil bağlantı için kernel'e doğru bir I/O request gerçekleşiyor. Üstelik,
    ana döngü iterasyonunda her defasında. Read / Write operasyonları sonuçta birer Syscall anlamına
    gelir. Syscall'ların da maliyeti yüksektir. Örneğin aktif istemci sayısının 1000 olduğunu ve
    sadece %5'inin hazır olduğunu düşünelim. Yine de Read/Write çağrısının 1000 kez yapıyor olacağız.
    Bu da main thread döngüsünün giderek verimsizleşmesi anlamına gelecektir.

    Bloklu I/O operasyonlarında işleri plana göre çalıştırmak daha verimlidir çünkü kaynakların hazır
    olduğunu biliriz. Non-Blocking I/O modunda ise bunu kontrol etmeden bilemeyiz ve kontrol operasyonu
    da gördüğünüz üzere pahalı.

    Buradaki dezavantaj için Linux tarafında epoll API yardımcı olabilir. epool-web-server örneği.

*/
use std::io::ErrorKind::WouldBlock;
use std::io::{Read, Write};
use std::net::TcpListener;

enum IOState<'a> {
    Read { request: [u8; 1024], read: usize },
    Write { response: &'a [u8], written: usize },
    Flush,
}

fn main() {
    let listener = TcpListener::bind("localhost:4000").expect("4000 nolu port bağlanamadı");
    listener
        .set_nonblocking(true)
        .expect("Non-Blocking moda geçilemedi");
    println!("Sunucu -> localhost:4000");
    let mut connections = Vec::new();
    loop {
        match listener.accept() {
            Ok((conn, _)) => {
                conn.set_nonblocking(true)
                    .expect("Non-blocking moda geçilemedi");
                let state = IOState::Read {
                    request: [0u8; 1024],
                    read: 0,
                };
                connections.push((conn, state));
            }
            Err(e) if e.kind() == WouldBlock => {}
            Err(e) => panic!("{}", e),
        };

        let mut completed = Vec::new();

        'next_connection: for (i, (conn, state)) in connections.iter_mut().enumerate() {
            match state {
                IOState::Read { request, read } => {
                    loop {
                        match conn.read(&mut request[*read..]) {
                            Ok(0) => {
                                println!("istemci beklenmedik şekilde sonlandı");
                                completed.push(i);
                                continue 'next_connection;
                            }
                            Ok(n) => {
                                *read += n;
                            }
                            Err(err) if err.kind() == WouldBlock => {
                                // bulunduğu loop değil de bir üst loop'a (for döngüsü) atlamak için
                                // 'next kullanılmıştır
                                continue 'next_connection;
                            }
                            Err(err) => panic!("{}", err),
                        }

                        if request.get(*read - 4..*read) == Some(b"\r\n\r\n") {
                            break;
                        }
                    }

                    let request = String::from_utf8_lossy(&request[..*read]);
                    println!("{request}");
                    let dummy_response = concat!(
                        "HTTP/1.1 200 OK\r\n",
                        "Content-Length: 17\n",
                        "Connection: close\r\n\r\n",
                        "EnCinX Server 1.0"
                    );

                    *state = IOState::Write {
                        response: dummy_response.as_bytes(),
                        written: 0,
                    };
                }
                IOState::Write { response, written } => {
                    loop {
                        match conn.write(&response[*written..]) {
                            Ok(0) => {
                                println!("istemci beklenmedik şekilde sonlandı");
                                completed.push(i);
                                continue 'next_connection;
                            }
                            Ok(n) => {
                                *written += n;
                            }
                            Err(e) if e.kind() == WouldBlock => {
                                continue 'next_connection;
                            }
                            Err(e) => panic!("{}", e),
                        }
                        if *written == response.len() {
                            break;
                        }
                    }
                    *state = IOState::Flush;
                }
                IOState::Flush => match conn.flush() {
                    Ok(_) => {
                        completed.push(i);
                    }
                    Err(e) if e.kind() == WouldBlock => {
                        continue 'next_connection;
                    }
                    Err(e) => panic!("{}", e),
                },
            }
        }

        for i in completed.into_iter().rev() {
            connections.remove(i);
        }
    }
}
