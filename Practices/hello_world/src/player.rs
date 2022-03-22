use crate::level::Level;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Player<'a> {
    pub name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    /// Yeni bir Oyuncu nesnesi örnekler.
    pub fn new(name: &'a str, level: Level) -> Self {
        Self { name, level }
    }
    // pub fn to_string(&self) -> String {
    //     //println!("{}{:?}", self.name, self.level);
    //     format!("{} -[{:?}]", self.name, self.level)
    // }
}

impl<'a> Display for Player<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -[{:?}]", self.name, self.level)
    }
}
