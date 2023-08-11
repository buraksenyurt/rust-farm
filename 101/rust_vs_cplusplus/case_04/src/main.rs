struct Player {
    name: String,
    level: i32,
}

impl Player {
    fn new(name: String, level: i32) -> Self {
        Self { name, level }
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let player = Player::new("Doktor Acayip".to_string(), 400);
    let dangling_player = &player;
    calc_bonus(player);
    println!("İşlemleri yapılan oyuncu {}", dangling_player.get_name());
}

fn calc_bonus(player: Player) {
    let player_name = player.get_name();
    println!("{player_name} için bonus hesaplamaları.");
}
