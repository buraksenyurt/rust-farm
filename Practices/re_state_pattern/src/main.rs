use re_state_pattern::game::Game;
use re_state_pattern::game_state::GameState;
use re_state_pattern::state::State;

fn main() {
    let mut dragon_fly = Game::new(1);
    dragon_fly.playing();
    println!("Güncel state {}", dragon_fly.state.get_state());
    dragon_fly.end_game();
    println!("Güncel state {}", dragon_fly.state.get_state());
    dragon_fly.play_again();
    println!("Güncel state {}",dragon_fly.state.get_state());
    dragon_fly.end_game();
    println!("Güncel state {}", dragon_fly.state.get_state());
    dragon_fly.menu();
    println!("Güncel state {}", dragon_fly.state.get_state());
    dragon_fly.playing();
    println!("Güncel state {}", dragon_fly.state.get_state());
    dragon_fly.init();
    println!("Güncel state {}", dragon_fly.state.get_state());
}
