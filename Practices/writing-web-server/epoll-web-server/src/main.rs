/*
    epoll-web-server

    linux tarafındaki epoll API'sini cover eden bir crate kullanılıyor.
    a-non-blocking-server'daki verimsizliği azaltmaya yardımcı olacak bir yöntem söz konusu.


    Dezavantaj;

    Linux epoll api bağımlılığı var. Windows tarafı ???
    Dikkat edileceği üzere kod karmaşıklığı artmıştır.
*/
use epoll::ControlOptions::EPOLL_CTL_ADD;
use epoll::{Event, Events};
use std::collections::HashMap;
use std::io::ErrorKind::WouldBlock;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::os::fd::AsRawFd;

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
    let epoll = epoll::create(false).expect("epoll api çalıştırılamadı");
    let event = Event::new(Events::EPOLLIN, listener.as_raw_fd() as _);
    epoll::ctl(epoll, EPOLL_CTL_ADD, listener.as_raw_fd(), event).expect("Epoll oluşturlamıyor");
    let mut connections = HashMap::new();

    println!("Sunucu -> localhost:4000");

    loop {
        let mut events = [Event::new(Events::empty(), 0); 1024];
        let block_timeout = -1;
        let event_count = epoll::wait(epoll, block_timeout, &mut events)
            .expect("bekleyen olay bildirimlerinde hata");
        println!("Event Count {}", event_count);

        let mut completed = Vec::new();

        'next_event: for event in &events[..event_count] {
            let file_descriptor = event.data as i32;

            // dinleyici hazırsa
            if file_descriptor == listener.as_raw_fd() {
                match listener.accept() {
                    Ok((connection, _)) => {
                        connection.set_nonblocking(true).unwrap();
                        let fd = connection.as_raw_fd();
                        let event = Event::new(Events::EPOLLIN | Events::EPOLLOUT, fd as _);
                        epoll::ctl(epoll, EPOLL_CTL_ADD, fd, event).unwrap();

                        let state = IOState::Read {
                            request: [0u8; 1024],
                            read: 0,
                        };

                        connections.insert(fd, (connection, state));
                    }
                    Err(e) if e.kind() == WouldBlock => {}
                    Err(err) => panic!("{}", err),
                }

                continue 'next_event;
            }

            let (conn, state) = connections
                .get_mut(&file_descriptor)
                .expect("Bağlantı alınırken hata");

            match state {
                IOState::Read { request, read } => {
                    loop {
                        match conn.read(&mut request[*read..]) {
                            Ok(0) => {
                                println!("istemci beklenmedik şekilde sonlandı");
                                completed.push(file_descriptor);
                                continue 'next_event;
                            }
                            Ok(n) => {
                                *read += n;
                            }
                            Err(err) if err.kind() == WouldBlock => {
                                continue 'next_event;
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
                                completed.push(file_descriptor);
                                continue 'next_event;
                            }
                            Ok(n) => {
                                *written += n;
                            }
                            Err(e) if e.kind() == WouldBlock => {
                                continue 'next_event;
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
                        completed.push(file_descriptor);
                    }
                    Err(e) if e.kind() == WouldBlock => {
                        continue 'next_event;
                    }
                    Err(e) => panic!("{}", e),
                },
            }
        }

        for file_descriptor in completed {
            let (conn, _) = connections.remove(&file_descriptor).unwrap();
            drop(conn);
        }
    }
}
