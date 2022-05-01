use std::fmt::{Display, Formatter};

pub enum BossLevel {
    Smurf,
    Gentle,
    Monstrous,
}

impl From<BossLevel> for u16 {
    fn from(level: BossLevel) -> Self {
        match level {
            BossLevel::Smurf => 30,
            BossLevel::Gentle => 20,
            BossLevel::Monstrous => 7,
        }
    }
}

impl Display for BossLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Smurf => write!(f, "Smurf mode on"),
            Self::Gentle => write!(f, "Gentle mode on"),
            Self::Monstrous => write!(f, "Monstrous mode on"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn boss_level_to_number_test() {
        assert_eq!(30, u16::from(BossLevel::Smurf));
        assert_eq!(20, u16::from(BossLevel::Gentle));
        assert_eq!(7, u16::from(BossLevel::Monstrous));
    }
}
