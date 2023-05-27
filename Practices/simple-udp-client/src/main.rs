use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("UDP soket oluşturulamadı.");
    socket
        .connect("127.0.0.1:5001")
        .expect("UDP sunucusuna bağlanılamıyor");
    println!("Peer adresi {:?}", socket.peer_addr());
    socket
        .send("Merhaba. Ben Dam.".as_bytes())
        .expect("Mesaj gönderilemedi");
    socket
        .send("Van Dam.".as_bytes())
        .expect("Mesaj gönderilemedi");
    socket
        .send("Claud Van Dam.".as_bytes())
        .expect("Mesaj gönderilemedi");
    socket
        .send("Jan Claud Van Dam.".as_bytes())
        .expect("Mesaj gönderilemedi");
}