use re_state_pattern::game::Game;
use re_state_pattern::game_state::GameState;

fn main() {
    let mut dragon_fly = Game::new(1);
    dragon_fly.playing();
    println!("{}", dragon_fly);
    dragon_fly.end_game();
    println!("{}", dragon_fly);
    dragon_fly.play_again();
    println!("{}", dragon_fly);
    dragon_fly.end_game();
    println!("{}", dragon_fly);
    dragon_fly.menu();
    println!("{}", dragon_fly);
    dragon_fly.playing();
    println!("{}", dragon_fly);
    dragon_fly.init();
    println!("{}", dragon_fly);
}
