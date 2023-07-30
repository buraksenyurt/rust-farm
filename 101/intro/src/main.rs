use std::fmt::{Display, Formatter};

fn main() {
    let mut players = vec![];

    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    players.push(wilson);
    let cesika = Player::new(32, "Jesica Abla", Level::Elit);
    players.push(cesika);
    let con = Player::new(13, "Con Wik", Level::Beginner(Score { win: 10, lose: 4 }));
    players.push(con);

    players.iter_mut().for_each(|p| {
        p.revenue = calculate_revenue(&p.level);
        println!(
            "{}({}) isimli oyuncunun ödülü {} coin",
            p.nick_name, p.level, p.revenue
        );
    });
}

fn calculate_revenue(level: &Level) -> i32 {
    let revenue = match level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            11..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };
    revenue
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
    pub revenue: i32,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
            revenue: 0,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Beginner(s) => {
                write!(f, "({:?})", s)
            }
            Level::Pro(s) => {
                write!(f, "({:?})", s)
            }
            Level::Veteran(s) => {
                write!(f, "({:?})", s)
            }
            Level::Elit => {
                write!(f, "(Elit)")
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Score {
    pub win: u16,
    pub lose: u16,
}