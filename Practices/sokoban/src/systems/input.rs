use crate::game_play::GamePlay;
use crate::prelude::*;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // Girdi sisteminin kullanacağı verileri SystemData tipinde topluyoruz.
    // Basılan tuşlar için bir kuyruk, lokasyon bilgisi için bir tane ve
    // son olarak da oyuncu verisini okuma amaçlı bir başkası.
    type SystemData = (
        Write<'a, InputEvents>,
        Write<'a, GamePlay>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut input_events,
            mut game_play,
            entities,
            mut positions,
            players,
            movables,
            immovables,
        ) = data;
        let mut candidates = Vec::new();
        for (position, _player) in (&positions, &players).join() {
            if let Some(key) = input_events.pressed_keys.pop() {
                // sahadaki hareket edebilir ve edemez tüm nesneler toplanır
                let movables: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let immovables: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };
                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    match movables.get(&pos) {
                        Some(id) => candidates.push((key, *id)),
                        None => match immovables.get(&pos) {
                            Some(_) => candidates.clear(),
                            None => break,
                        },
                    }
                }
            }
        }

        // Oyuncunun hareket alanı varsa oyun durumunu tutan nesnedeki ilgili değeri artır.
        if candidates.is_empty() {
            game_play.moves_count += 1;
        }

        for (key, id) in candidates {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }

        // // Sistem gelen veriyi data parametresi üstünden alıyoruz
        // let (mut input_events, mut positions, players) = data;
        //
        // // konumlar ve oyuncu bilgilerini bir arada ele alan bir döngü
        // for (position, _player) in (&mut positions, &players).join() {
        //     // Eğer basılan tuş match ifadesindekilerden birisiyse buna göre
        //     // x,y konum bilgisini değiştiriyoruz
        //     if let Some(key) = input_events.pressed_keys.pop() {
        //         match key {
        //             KeyCode::Up => position.y -= 1,
        //             KeyCode::Down => position.y += 1,
        //             KeyCode::Left => position.x -= 1,
        //             KeyCode::Right => position.x += 1,
        //             _ => (),
        //         }
        //     }
        // }
    }
}
