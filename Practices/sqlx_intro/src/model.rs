use std::fmt::{Display, Formatter};

pub struct Category {
    pub id: i64,
    pub name: String,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.id, self.name)
    }
}
