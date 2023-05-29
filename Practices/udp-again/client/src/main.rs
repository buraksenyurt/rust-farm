use std::error::Error;
use bincode::{deserialize, serialize};
use tokio::net::UdpSocket;
use common_lib::{GameState, PlayerState};

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    let server_ip_port="127.0.0.1:5002";
    let socket=UdpSocket::bind(server_ip_port).await.expect("Soket oluşturulamıyor");
    let player_state=PlayerState::new(19.10,10.50,20.45,1551);
    let encoded=serialize(&player_state).expect("Veri serileştirilemiyor");
    socket.send(&encoded).await.expect("Mesaj gönderilemedi");

    loop {
        let mut buffer = vec![0u8; 1024];
        let (_, _) = socket.recv_from(&mut buffer).await?;
        let game_state: GameState = deserialize(&buffer)?;
        for ps in game_state.players {
            println!("{}", ps);
        }
    }
}
