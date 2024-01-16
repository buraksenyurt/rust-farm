use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
pub struct Player<'a> {
    id: i32,
    title: &'a str,
    point: f32,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, title: &'a str, point: f32) -> Self {
        Self { id, title, point }
    }
}

impl<'a> PartialEq<Self> for Player<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.point.eq(&other.point)
    }
}

impl<'a> PartialOrd for Player<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.point.partial_cmp(&other.point)
    }
}
