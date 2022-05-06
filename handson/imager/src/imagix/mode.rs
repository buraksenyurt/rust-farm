use crate::error::ImagixError;
use std::str::FromStr;

/// Tek dosya için mi n sayıda dosya için mi işlem yapılacağını belirten mod
#[derive(Debug)]
pub enum Mode {
    Single,
    Full,
}

impl FromStr for Mode {
    type Err = ImagixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Mode::Single),
            "full" => Ok(Mode::Full),
            _ => Ok(Mode::Single),
        }
    }
}
