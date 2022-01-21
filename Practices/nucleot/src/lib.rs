use crate::DnaError::Invalid;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn valid_dna_sequence_test() {
        let result = check_dna("GATTACA").unwrap();
        assert_eq!(result, "başarılı");
    }
}

pub fn check_dna(sequence: &str) -> Result<&str, DnaError> {
    Ok("başarılı")
}

impl Error for DnaError {}

impl Display for DnaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Invalid => write!(f, "Invalid"),
        }
    }
}

#[derive(Debug)]
pub enum DnaError {
    Invalid,
}
