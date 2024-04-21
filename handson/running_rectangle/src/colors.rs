use rand::seq::SliceRandom;

enum Color {
    IndianRed,
    MediumVioletRed,
    Tomato,
    Coral,
    Gold,
    Indigo,
    Lime,
    Olive,
    Teal,
    SteelBlue,
    MediumSlateBlue,
    NavajoWhite,
    Chocolate,
    MistyRose,
    MidnightBlue,
    BurlyWood,
    Aquamarine,
}

impl Color {
    fn to_hex(&self) -> &str {
        match *self {
            Color::IndianRed => "#CD5C5C",
            Color::MediumVioletRed => "#C71585",
            Color::Tomato => "#FF6347",
            Color::Coral => "#FF7F50",
            Color::Gold => "#FFD700",
            Color::Indigo => "#4B0082",
            Color::Lime => "#00FF00",
            Color::Olive => "#808000",
            Color::Teal => "#008080",
            Color::SteelBlue => "#4682B4",
            Color::MediumSlateBlue => "#7B68EE",
            Color::NavajoWhite => "#FFDEAD",
            Color::Chocolate => "#D2691E",
            Color::MistyRose => "#FFE4E1",
            Color::MidnightBlue => "#191970",
            Color::BurlyWood => "#DEB887",
            Color::Aquamarine => "#7FFFD4",
        }
    }
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
        Color::Lime,
        Color::Olive,
        Color::Teal,
        Color::SteelBlue,
        Color::MediumSlateBlue,
        Color::NavajoWhite,
        Color::Chocolate,
        Color::MistyRose,
        Color::MidnightBlue,
        Color::BurlyWood,
        Color::Aquamarine,
    ];
    colors.choose(&mut rng).unwrap().to_hex().to_string()
}
