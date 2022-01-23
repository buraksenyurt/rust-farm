#[derive(Debug)]
pub struct Player {
    life: u8,
    nickname: String,
    universe: String,
    is_active: bool,
}

impl Player {
    pub fn new(nickname: String) -> Self {
        Player {
            life: 3,
            nickname,
            is_active: false,
            universe: String::from("nowhere"),
        }
    }

    pub fn connect(&mut self, universe: String) {
        if !self.is_active && self.life > 0 {
            self.is_active = true;
            self.universe = universe;
        }
    }

    pub fn hited(&mut self) {
        self.life -= 1;
        if self.life == 0 {
            self.is_active = false;
        }
    }
}
