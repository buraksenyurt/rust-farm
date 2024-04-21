pub enum Color {
    IndianRed,
    Crimson,
    MediumVioletRed,
    Tomato,
    Coral,
    Gold,
    Indigo,
    LimeGreen,
    Olive,
    Teal,
    SteelBlue,
    MediumSlateBlue,
    SandyBrown,
    Chocolate,
    FireBrick,
    MidnightBlue,
    BurlyWood,
    CadetBlue,
}

impl Color {
    pub fn to_hex(&self) -> &str {
        match *self {
            Color::IndianRed => "#CD5C5C",
            Color::MediumVioletRed => "#C71585",
            Color::Crimson => "#DC143C",
            Color::Tomato => "#FF6347",
            Color::Coral => "#FF7F50",
            Color::Gold => "#FFD700",
            Color::Indigo => "#4B0082",
            Color::LimeGreen => "#32CD32",
            Color::Olive => "#808000",
            Color::Teal => "#008080",
            Color::SteelBlue => "#4682B4",
            Color::MediumSlateBlue => "#7B68EE",
            Color::SandyBrown => "#F4A460",
            Color::Chocolate => "#D2691E",
            Color::FireBrick => "#B22222",
            Color::MidnightBlue => "#191970",
            Color::BurlyWood => "#DEB887",
            Color::CadetBlue => "#5F9EA0",
        }
    }
}
