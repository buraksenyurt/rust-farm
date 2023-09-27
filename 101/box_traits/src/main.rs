fn main() {
    let player: Box<dyn GameObject> = Box::new(Player {
        name: String::from("My Hero"),
        health: 100,
    });

    let enemy: Box<dyn GameObject> = Box::new(Enemy {
        kind: String::from("Ork"),
        strength: 45,
    });

    let game_objects: Vec<Box<dyn GameObject>> = vec![player, enemy];

    for obj in game_objects {
        obj.draw();
    }
}

trait GameObject {
    fn draw(&self);
}

struct Player {
    name: String,
    health: i32,
}
struct Enemy {
    kind: String,
    strength: i32,
}

impl GameObject for Player {
    fn draw(&self) {
        println!("Oyuncu: {} (Sağlık: {})", self.name, self.health);
    }
}

impl GameObject for Enemy {
    fn draw(&self) {
        println!("Düşman: {} (Güç: {})", self.kind, self.strength);
    }
}

