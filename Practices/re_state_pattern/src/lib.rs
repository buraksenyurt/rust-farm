/*
   Bu örnekte bir Oyun nesnesinin Init, Playing, PlayAgain, EndGame konumlarında olma halini ele aldığımız
   bir state pattern örneği söz konusu.

   Oyun nesnesi Init konumundan, Playing konumuna geçebilir ama EndGame konumuna geçemez.
   Playing konumundan EndGame konumuna geçebilir ama Init konumuna geçemez.
   EndGame konumundan PlayAgain ve Menu konumuna geçebilir.
*/
pub mod end_game;
pub mod game;
pub mod game_state;
pub mod init_state;
pub mod menu_state;
pub mod play_again;
pub mod playing;
pub mod state;

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::game_state::GameState;
    use crate::state::State;

    #[test]
    fn should_game_on_new_state_test() {
        let dragon_fly = Game::new(1);
        let current_state = dragon_fly.get_state();
        assert_eq!(current_state, "".to_string());
    }

    // Oyun nesnesi Init konumundan, Playing konumuna geçebilir ama EndGame konumuna geçemez.
    #[test]
    fn should_game_on_playing_state_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        assert_eq!(dragon_fly.state.get_state(), "PLAYING".to_string());
    }

    // Playing konumundan EndGame konumuna geçebilir.
    #[test]
    fn should_game_on_endgame_state_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.end_game();
        assert_eq!(dragon_fly.state.get_state(), "END_GAME".to_string());
    }

    // Playing konumundan Init konumuna geçemez.
    #[test]
    fn game_should_not_be_init_state_when_playing_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.init();
        assert_eq!(dragon_fly.state.get_state(), "PLAYING".to_string());
    }

    // EndGame konumundan PlayAgain konumuna geçebilir.
    #[test]
    fn should_game_on_play_again_state_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.end_game();
        dragon_fly.play_again();
        assert_eq!(dragon_fly.state.get_state(), "PLAY_AGAIN".to_string());
    }

    // EndGame konumundan Menu konumuna geçebilir.
    #[test]
    fn should_game_on_menu_state_after_end_game_test() {
        let mut dragon_fly = Game::new(1);
        dragon_fly.playing();
        dragon_fly.end_game();
        dragon_fly.menu();
        assert_eq!(dragon_fly.state.get_state(), "MENU".to_string());
    }
}
