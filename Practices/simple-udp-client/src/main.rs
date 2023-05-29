mod message;

use crate::message::Message;
use bincode::serialize;
use std::net::UdpSocket;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("UDP soket oluşturulamadı.");
    socket
        .connect("127.0.0.1:5001")
        .expect("UDP sunucusuna bağlanılamıyor");
    println!("Peer adresi {:?}", socket.peer_addr());
    send_message(&socket, Message::new(rng.gen_range(1..10), "Merhaba. Ben Dam.".to_string()));
    send_message(&socket, Message::new(rng.gen_range(10..11), "Van Dam.".to_string()));
    send_message(&socket, Message::new(rng.gen_range(11..20), "Claud Van Dam.".to_string()));
    send_message(&socket, Message::new(rng.gen_range(21..30), "Jan Claud Van Dam.".to_string()));
}

fn send_message(socket: &UdpSocket, message: Message) {
    let mut buffer = [0; 1024];
    let encoded_message = serialize(&message).expect("Serileştirme başarısız");
    socket.send(&encoded_message).expect("Mesaj gönderilemedi");

    match socket.recv_from(&mut buffer) {
        Ok((size, _)) => {
            let response = String::from_utf8_lossy(&buffer[..size]);
            println!("Sunucudan der ki\n\t{}", response);
        }
        Err(_) => {}
    }
}
