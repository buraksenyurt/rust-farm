use crate::colors::Color;
use rand::prelude::SliceRandom;
use crate::entity::BlockSize;
use crate::instrument::Size;

pub struct Utility {}

impl Utility {
    pub fn get_random_size() -> Size {
        let mut rng = rand::thread_rng();
        let block_sizes = [
            BlockSize::Short,
            BlockSize::Tall,
            BlockSize::Grande,
            BlockSize::Venti,
        ];
        block_sizes.choose(&mut rng).unwrap().to_size()
    }

    pub fn get_random_color() -> String {
        let mut rng = rand::thread_rng();
        let colors = [
            Color::IndianRed,
            Color::MediumVioletRed,
            Color::Tomato,
            Color::Coral,
            Color::Gold,
            Color::Indigo,
            Color::LimeGreen,
            Color::Olive,
            Color::Teal,
            Color::SteelBlue,
            Color::MediumSlateBlue,
            Color::SandyBrown,
            Color::Chocolate,
            Color::FireBrick,
            Color::MidnightBlue,
            Color::BurlyWood,
            Color::CadetBlue,
            Color::Crimson,
        ];
        colors.choose(&mut rng).unwrap().to_hex().to_string()
    }
}
