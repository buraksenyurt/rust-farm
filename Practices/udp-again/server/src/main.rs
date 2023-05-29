use bincode::{deserialize, serialize};
use common_lib::{GameState, PlayerState};
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let game_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState::default()));
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
        let mut guarded_game_state = game_state.lock().expect("Mutex tarafında sorun var");
        let is_exist = guarded_game_state
            .players
            .iter()
            .any(|p| p.player_id == ps.player_id);
        if !is_exist {
            guarded_game_state.players.push(ps);
        }

        //TODO Oyuncu durumlarını burada olan tüm istemcilere göndermek lazım
        let player_states = serialize(&guarded_game_state.players).expect("Veri serileştirilemedi");
        socket
            .send_to(&player_states, client)
            .await
            .expect("Oyun durumu gönderilemedi");
        println!("[Client {}:{}] : \n{}", client.ip(),client.port(), ps.clone());
    }
}
