use bincode::{deserialize, serialize};
use common_lib::{GameState, PlayerState};
use rand::Rng;
use std::error::Error;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .expect("Soket oluşturulamıyor");
    let player_state = PlayerState::new(
        rng.gen_range(0_f32..10.0),
        rng.gen_range(0_f32..10.0),
        rng.gen_range(10_f32..32.0),
        rng.gen_range(1..32500),
    );
    let encoded = serialize(&player_state).expect("Veri serileştirilemiyor");
    socket
        .send_to(&encoded, "127.0.0.1:5002")
        .await
        .expect("Mesaj gönderilemedi");
    loop {
        let mut buffer = vec![0u8; 1024];
        let (_, _) = socket
            .recv_from(&mut buffer)
            .await
            .expect("Sunucudan veri alınamıyor");
        let game_state: GameState = deserialize(&buffer).expect("Ters serileştirmede sorun var");
        for ps in game_state.players {
            println!("{}", ps);
        }
    }
}
