#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (idx, c) in dna.chars().enumerate() {
            match c {
                'G' | 'C' | 'T' | 'A' => continue,
                _ => return Err(idx),
            }
        }
        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        let mut sequence = String::new();
        for c in self.0.chars() {
            match c {
                'G' => sequence.push('C'),
                'C' => sequence.push('G'),
                'T' => sequence.push('A'),
                'A' => sequence.push('U'),
                _ => {}
            }
        }
        Rna(sequence)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (idx, c) in rna.chars().enumerate() {
            match c {
                'C' | 'G' | 'A' | 'U' => continue,
                _ => return Err(idx),
            }
        }
        Ok(Rna(rna.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rna_sequence() {
        let input = "";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn rna_complement_of_cytosine_is_guanine() {
        let input = "C";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("G").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn rna_complement_of_guanine_is_cytosine() {
        let input = "G";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("C").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn rna_complement_of_thymine_is_adenine() {
        let input = "T";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("A").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn rna_complement_of_adenine_is_uracil() {
        let input = "A";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("U").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn rna_complement() {
        let input = "ACGTGGTCTTAA";
        let output = Dna::new(input).unwrap().into_rna();
        let expected = Rna::new("UGCACCAGAAUU").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn invalid_dna_input() {
        let input = "U";
        let output = Dna::new(input);
        let expected = Err(0);
        assert_eq!(output, expected);
    }

    #[test]
    fn invalid_dna_input_at_offset() {
        let input = "ACGTUXXCTTAA";
        let output = Dna::new(input);
        let expected = Err(4);
        assert_eq!(output, expected);
    }

    #[test]
    fn invalid_rna_input() {
        let input = "T";
        let output = Rna::new(input);
        let expected = Err(0);
        assert_eq!(output, expected);
    }

    #[test]
    fn invalid_rna_input_at_offset() {
        let input = "ACGTUXXCTTAA";
        let output = Rna::new(input);
        let expected = Err(3);
        assert_eq!(output, expected);
    }
}
