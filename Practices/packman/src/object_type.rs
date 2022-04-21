#[derive(Copy, Clone, PartialEq)]
pub enum ObjectType {
    Floor,
    Wall,
    Apple(usize),
    RottenApple(usize),
}
