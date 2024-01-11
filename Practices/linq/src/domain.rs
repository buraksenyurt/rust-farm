#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Genre {
    Action,
    Adventure,
    Puzzle,
    Racing,
    Simulation,
    Strategy,
    SciFi,
    Horror,
    RPG,
    Fantasy,
    Shooter,
    Sports,
}

pub enum Order {
    Ascending,
    Descending,
}

#[derive(Clone, PartialEq)]
pub enum Expertise {
    AI,
    Graphics,
    LevelDesign,
    Gameplay,
    Physics,
    Story,
}
#[derive(Clone)]
pub struct Programmer {
    pub name: String,
    pub expertise: Expertise,
}

#[derive(Clone)]
pub struct Game {
    pub name: String,
    pub genre: Genre,
    pub rating: f32,
    pub release_year: usize,
    pub programmer: Vec<Programmer>,
}
