use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Power(u8);

impl Power {
    pub fn new(value: u8) -> Self {
        Power(value)
    }
}

impl Default for Power {
    fn default() -> Self {
        Power(10)
    }
}

impl Display for Power {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mana", self.0)
    }
}
