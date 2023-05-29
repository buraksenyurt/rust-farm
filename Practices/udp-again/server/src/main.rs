use bincode::deserialize;
use common_lib::{GameState, PlayerState};
use std::error::Error;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut game_state = GameState::default();
    let ip_port = "127.0.0.1:5002";
    let socket = UdpSocket::bind(ip_port)
        .await
        .expect("Soket oluşturulamıyor");
    println!("Soket hazır.{}", ip_port);

    loop {
        let mut buffer = vec![0u8; 512];
        let (_, client) = socket
            .recv_from(&mut buffer)
            .await
            .expect("Gelen veri okunamıyor");
        let ps: PlayerState = deserialize(&buffer).expect("Gelen veri çözümlenemiyor");
        let is_exist = game_state
            .players
            .iter()
            .any(|p| p.player_id == ps.player_id);
        if !is_exist {
            game_state.players.push(ps);
        }
        println!("Client{}:\n{}", client.ip(), ps.clone());
    }
}
