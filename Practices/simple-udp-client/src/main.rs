use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("UDP soket oluşturulamadı.");
    socket
        .connect("127.0.0.1:5001")
        .expect("UDP sunucusuna bağlanılamıyor");
    println!("Peer adresi {:?}", socket.peer_addr());
    send_message(&socket, "Merhaba. Ben Dam.");
    send_message(&socket, "Van Dam.");
    send_message(&socket, "Claud Van Dam.");
    send_message(&socket, "Jan Claud Van Dam.");
}

fn send_message(socket: &UdpSocket, message: &str) {
    let mut buffer = [0; 1024];

    socket
        .send(message.as_bytes())
        .expect("Mesaj gönderilemedi");

    match socket.recv_from(&mut buffer) {
        Ok((size, _)) => {
            let response = String::from_utf8_lossy(&buffer[..size]);
            println!("Sunucudan der ki\n\t{}", response);
        }
        Err(_) => {}
    }
}
