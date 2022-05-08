use crate::game_play::GamePlay;
use crate::game_state::GameState;
use crate::prelude::*;

// Oyunun durum yönetim sistemi
pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, GamePlay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Chest>,
        ReadStorage<'a, ChestSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_play_state, positions, chests, chest_spots) = data;

        let chests_by_position: HashMap<(u8, u8), &Chest> = (&positions, &chests)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();

        for (_, position) in (&chest_spots, &positions).join() {
            if chests_by_position.contains_key(&(position.x, position.y)) {
                continue;
            } else {
                game_play_state.state = GameState::Play;
                return;
            }
        }
        game_play_state.state = GameState::Win;
    }
}
