#[derive(Debug)]
pub struct Voyager {
    pub life: u8,
    pub nickname: String,
    pub universe: String,
    pub is_active: bool,
}

impl Voyager {
    pub fn new(nickname: String) -> Self {
        Voyager {
            nickname,
            ..Default::default()
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

impl Default for Voyager {
    fn default() -> Self {
        Voyager {
            life: 3,
            is_active: false,
            universe: String::from("nowhere"),
            nickname: String::from("unknown"),
        }
    }
}
