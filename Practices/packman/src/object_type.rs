#[derive(Copy, Clone, PartialEq)]
pub enum ObjectType {
    Floor,
    Wall,
    Apple(usize),
    RottenApple(usize),
}

#[derive(Copy, Clone, PartialEq)]
pub enum FruitType {
    Apple,
    RottenApple,
}
