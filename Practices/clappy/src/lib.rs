use clap::{ArgEnum, PossibleValue};
use std::fmt::Display;
use std::str::FromStr;

// Terminalden girilecek format bilgilerini aşağıdaki enum sabitinde tutabiliriz
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Format {
    Json,
    Bson,
    Xml,
    Binary,
}

// clap içerisinde yer alan possible_values fonksiyonunu override ediyoruz.
impl Format {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        Format::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("Aranan değer yok")
            .get_name()
            .fmt(f)
    }
}

// FromStr trait'ini Format enum sabiti için yeniden uyarladık
// Böylece bir string değerden Format değerine çevrim yapılabilir ki bunu clap kullanmakta.
impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Geçersiz değer: {}", s))
    }
}
