mod level;
mod player;

use crate::level::Level;
use crate::player::Player;
use log::{error, info};
use std::io::stdin;
use std::process::exit;

fn main() {
    env_logger::init();

    let mut player_name = String::new();
    println!("Bana adını söyler misin?");
    let result = stdin().read_line(&mut player_name);
    match result {
        Ok(l) => {
            info!("{} byte veri alındı.", l)
        }
        Err(e) => {
            error!("{}", e);
            exit(-1)
        }
    }
    //println!("{}", name);

    // let persival = Player {
    //     name: player_name.trim(),
    //     level: Level::Beginner,
    // };
    let mut persival = Player::new(player_name.trim(), Level::Beginner);
    println!("{}", persival);
    apply_promotion(&mut persival);
    println!("{}", persival);
}

fn apply_promotion(p: &mut Player) {
    p.level = Level::Senior;
    println!("{}", p);
}
