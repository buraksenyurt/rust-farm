#[derive(Copy, Clone, PartialEq)]
pub enum ObjectType {
    Floor,
    Wall(usize),
    Apple(usize),
    RottenApple(usize),
}

#[derive(Copy, Clone, PartialEq)]
pub enum FruitType {
    Apple,
    RottenApple,
}
