struct Player {
    title: String,
    point: i32,
}

impl Player {
    fn new(title: String, point: i32) -> Self {
        Self { title, point }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_point(&self) -> i32 {
        self.point
    }
}

fn main() {
    let player = Player::new("Lorna".to_string(), 55);
    delete(player);
    // Ownership kuralları gereği derleme zamanında hata alınır
    let player_title = player.get_title(); // borrow of moved value: `player`
    let player_point = player.get_point(); // borrow of moved value: `player`
    println!("{player_title}({player_point})");
}

fn delete(player: Player) {
    let point = player.point;
    let title = player.title;
    println!("{title}{point}");
}
