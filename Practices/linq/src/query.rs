use crate::domain::{Expertise, Game, Genre, Programmer};

pub struct GameQueryEngine {
    pub games: Vec<Game>,
}

impl GameQueryEngine {
    pub fn get_by_rate_gte(&self, rate: f32) -> Vec<Game> {
        self.games
            .iter()
            .filter(|&g| g.rating >= rate)
            .cloned()
            .collect()
    }

    pub fn init() -> Self {
        Self {
            games: vec![
                Game {
                    name: "Space Adventure".to_string(),
                    genre: Genre::Action,
                    rating: 4.5,
                    programmer: vec![
                        Programmer {
                            name: "Alicey Johnson".to_string(),
                            expertise: Expertise::AI,
                        },
                        Programmer {
                            name: "Bob Smith".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                    ],
                },
                Game {
                    name: "Speed Racer".to_string(),
                    genre: Genre::Racing,
                    rating: 4.7,
                    programmer: vec![
                        Programmer {
                            name: "Chlie Brown".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                        Programmer {
                            name: "Alicey Johnson".to_string(),
                            expertise: Expertise::AI,
                        },
                        Programmer {
                            name: "Ben Johnsyn".to_string(),
                            expertise: Expertise::Graphics,
                        },
                    ],
                },
                Game {
                    name: "Mystic Quest".to_string(),
                    genre: Genre::Adventure,
                    rating: 4.2,
                    programmer: vec![
                        Programmer {
                            name: "David Lee".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                        Programmer {
                            name: "Alicey Johnson".to_string(),
                            expertise: Expertise::AI,
                        },
                        Programmer {
                            name: "Eevva Green".to_string(),
                            expertise: Expertise::Physics,
                        },
                        Programmer {
                            name: "Oliiva Wild".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                    ],
                },
                Game {
                    name: "Future Shock".to_string(),
                    genre: Genre::Shooter,
                    rating: 4.8,
                    programmer: vec![Programmer {
                        name: "Monaa Lissa".to_string(),
                        expertise: Expertise::Gameplay,
                    }],
                },
                Game {
                    name: "Battlefield Glory".to_string(),
                    genre: Genre::Shooter,
                    rating: 4.3,
                    programmer: vec![Programmer {
                        name: "Natahan Darke".to_string(),
                        expertise: Expertise::Gameplay,
                    }],
                },
                Game {
                    name: "Magic Tower".to_string(),
                    genre: Genre::Puzzle,
                    rating: 3.9,
                    programmer: vec![Programmer {
                        name: "Umaa Turman".to_string(),
                        expertise: Expertise::Story,
                    }],
                },
                Game {
                    name: "Race Challenge".to_string(),
                    genre: Genre::Racing,
                    rating: 4.1,
                    programmer: vec![
                        Programmer {
                            name: "Monaa Lissa".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                        Programmer {
                            name: "Umaa Turman".to_string(),
                            expertise: Expertise::Story,
                        },
                    ],
                },
                Game {
                    name: "Deep Dive".to_string(),
                    genre: Genre::Adventure,
                    rating: 4.4,
                    programmer: vec![Programmer {
                        name: "Viktor Hogo".to_string(),
                        expertise: Expertise::Gameplay,
                    }],
                },
                Game {
                    name: "Soccer Sim".to_string(),
                    genre: Genre::Sports,
                    rating: 3.8,
                    programmer: vec![
                        Programmer {
                            name: "Viktor Hogo".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                        Programmer {
                            name: "Lee Buruse".to_string(),
                            expertise: Expertise::Story,
                        },
                    ],
                },
                Game {
                    name: "Galaxy Invaders".to_string(),
                    genre: Genre::Action,
                    rating: 4.6,
                    programmer: vec![
                        Programmer {
                            name: "The Chosen Bill".to_string(),
                            expertise: Expertise::Gameplay,
                        },
                        Programmer {
                            name: "Lee Buruse".to_string(),
                            expertise: Expertise::Story,
                        },
                        Programmer {
                            name: "Donna Roni".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                    ],
                },
                Game {
                    name: "Treasure Island".to_string(),
                    genre: Genre::Adventure,
                    rating: 4.5,
                    programmer: vec![
                        Programmer {
                            name: "Viktor Hogo".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                        Programmer {
                            name: "Donna Roni".to_string(),
                            expertise: Expertise::LevelDesign,
                        },
                    ],
                },
            ],
        }
    }
}
