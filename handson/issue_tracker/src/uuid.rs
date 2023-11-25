use crate::utility::Utility;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Uuid {
    pub value: String,
}

impl Uuid {
    pub fn new() -> Self {
        let value = Utility::gen_guid();
        Self { value }
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
