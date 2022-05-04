use crate::prelude::*;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // Girdi sisteminin kullanacağı verileri SystemData tipinde topluyoruz.
    // Basılan tuşlar için bir kuyruk, lokasyon bilgisi için bir tane ve
    // son olarak da oyuncu verisini okuma amaçlı bir başkası.
    type SystemData = (
        Write<'a, InputEvents>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // Sistem gelen veriyi data parametresi üstünden alıyoruz
        let (mut input_events, mut positions, players) = data;

        // konumlar ve oyuncu bilgilerini bir arada ele alan bir döngü
        for (position, _player) in (&mut positions, &players).join() {
            // Eğer basılan tuş match ifadesindekilerden birisiyse buna göre
            // x,y konum bilgisini değiştiriyoruz
            if let Some(key) = input_events.pressed_keys.pop() {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}
