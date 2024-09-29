use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub struct Health(u32);
impl Health {
    pub fn new(value: u32) -> Self {
        Health(value)
    }
    pub fn lose(&mut self, amount: u32) {
        self.0 -= amount;
    }
}

impl Default for Health {
    fn default() -> Self {
        Health::new(100)
    }
}

impl Display for Health {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", **self)
    }
}

impl Deref for Health {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
