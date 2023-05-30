use bincode::{deserialize, serialize};
use common_lib::{GameState, PlayerState};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let game_state: Arc<Mutex<GameState>> = Arc::new(Mutex::new(GameState::default()));
    let clients: Arc<Mutex<Vec<SocketAddr>>> = Arc::new(Mutex::new(vec![]));
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

        clients.lock().await.push(client);

        let ps: PlayerState;
        match deserialize(&buffer) {
            Ok(dsr) => {
                ps = dsr;
                println!("[Client {}:{}] : \n{}", client.ip(), client.port(), dsr);
            }
            Err(e) => {
                println!("Veri çözümlenemiyor...{}", e);
                continue;
            }
        }

        let mut guarded_game_state = game_state.lock().await;
        let is_exist = guarded_game_state
            .players
            .iter()
            .any(|p| p.player_id == ps.player_id);
        if !is_exist {
            guarded_game_state.players.push(ps);
        }

        let player_states = serialize(&guarded_game_state.players).expect("Veri serileştirilemedi");
        for c in clients.lock().await.iter() {
            socket
                .send_to(&player_states, c)
                .await
                .expect("Oyun durumu gönderilemedi");
        }
    }
}
