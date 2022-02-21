#[allow(dead_code)]

#[derive(Debug)]
struct Player {
    id: u32,
    nick: String,
    country: String,
    level: u16,
}

impl Player {
    fn new(id: u32, nick: String, country: String, level: u16) -> Self {
        Self {
            id,
            nick,
            country,
            level,
        }
    }
}

fn main() {
    let gonzi = Player::new(1, "Gonsalez".to_string(), "Brasil".to_string(), 88);
    println!("{:#?}", gonzi);
}
