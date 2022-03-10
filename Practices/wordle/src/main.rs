use colored::Colorize;
use wordle::Manager;

fn main() {
    let wellcome = "World oyununun klonuna hoş geldiniz.\nHaydi başlayalım.".yellow();
    println!("{}", wellcome);

    let mut poe = Manager::new();
    loop {
        poe.draw_board();
        let user_guess = poe.take_guess();
        if poe.is_it_over(&user_guess) {
            break;
        }
    }
}
