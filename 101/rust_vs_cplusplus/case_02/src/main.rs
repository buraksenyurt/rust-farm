struct Player {
    title: String,
    point: i32,
}

impl Player {
    fn new(title: String, point: i32) -> Self {
        Self { title, point }
    }
}

fn main() {
    let player = Player::new("Dolares".to_string(), 67);
    do_something(player);

    do_something(player);
}

fn do_something(player: Player) {
    println!("{}-{}", player.title, player.point);
}
