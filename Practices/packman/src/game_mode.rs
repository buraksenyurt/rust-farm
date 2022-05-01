use std::fmt::{Display, Formatter};

pub enum GameMode {
    Menu,
    Playing,
    Options,
    End,
}

impl Display for GameMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Playing => write!(f, "Game Mode -> Playing"),
            Self::Menu => write!(f, "Game Mode -> Menu"),
            Self::Options => write!(f, "Game Mode -> Options"),
            Self::End => write!(f, "Game Mode -> End"),
        }
    }
}
