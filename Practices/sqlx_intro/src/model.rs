use std::fmt::{Display, Formatter};

pub struct Category {
    pub id: i64,
    pub title: String,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.id, self.title)
    }
}

pub struct Product {
    pub id: i64,
    pub title: String,
    pub category_id: i64,
    pub unit_price: f32,
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}({} Lira)", self.id, self.title, self.unit_price)
    }
}
