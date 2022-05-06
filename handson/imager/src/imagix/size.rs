use crate::error::ImagixError;
use std::str::FromStr;

/// İmge dönüştürmede kullanılan boyut
#[derive(Debug)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl From<Size> for u32 {
    fn from(size: Size) -> Self {
        match size {
            Size::Small => 50_u32,
            Size::Medium => 100_u32,
            Size::Large => 200_u32,
        }
    }
}

// Komut satırından structopt ile size ve mode gibi parametreleri alacağız.
// Bu String bilginin uygun Size enum değerine dönüştürülmesinde structopt, FromStr traitini kullanır.
impl FromStr for Size {
    type Err = ImagixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "small" => Ok(Size::Small),
            "medium" => Ok(Size::Medium),
            "large" => Ok(Size::Large),
            _ => Ok(Size::Small),
        }
    }
}
