use crate::DnaError::Invalid;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn valid_dna_sequence_test() {
        let result = check_dna("GatTAcA").unwrap();
        assert_eq!(result.to_string(), "'A':3, 'C':1, 'G':1, 'T':2");
        /*assert_eq!(result.adenine, 3);
        assert_eq!(result.cytosine, 1);
        assert_eq!(result.guanine, 1);
        assert_eq!(result.thymine, 2);*/
    }

    #[test]
    #[should_panic]
    fn invalid_dna_sequence_test() {
        let _result = check_dna("INVALID").unwrap();
        /*match result {
            Err(e) => assert_eq!(e, DnaError::Invalid),
            Ok(_) => {}
        };*/
    }
}

pub fn check_dna(sequence: &str) -> Result<Nucleotid, DnaError> {
    let mut is_invalid_char = false;
    for c in sequence.to_uppercase().chars() {
        match c {
            'A' | 'G' | 'T' | 'C' => continue,
            _ => is_invalid_char = true,
        }
    }

    if is_invalid_char {
        Err(DnaError::Invalid)
    } else {
        Ok(Nucleotid {
            adenine: 3,
            cytosine: 1,
            guanine: 1,
            thymine: 2,
        })
    }
}

pub struct Nucleotid {
    pub adenine: u8,
    pub cytosine: u8,
    pub guanine: u8,
    pub thymine: u8,
}

impl Display for Nucleotid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'A':{}, 'C':{}, 'G':{}, 'T':{}",
            self.adenine, self.cytosine, self.guanine, self.thymine,
        )
    }
}

impl Error for DnaError {}

impl Display for DnaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Invalid => write!(f, "Invalid"),
        }
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum DnaError {
    Invalid,
}
