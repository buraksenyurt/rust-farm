#[derive(Clone)]
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

#[derive(Clone)]
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
    pub programmer: Vec<Programmer>,
}
