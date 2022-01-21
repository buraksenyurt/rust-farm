#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn valid_rngsum_test() {
        // unwrap ile Result içerisindeki Ok değerini alırız.
        let result = rngsum(1, 6).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn invalid_range_rngsum_test() {
        let result = rngsum(6, 1);
        match result {
            Err(e) => assert_eq!(e, CalculationError::InvalidRange),
            Ok(_) => {}
        }
    }

    #[test]
    fn invalid_spacing_rngsum_test() {
        let result = rngsum(6, 6);
        match result {
            Err(e) => assert_eq!(e, CalculationError::InvalidSpacing),
            Ok(_) => {}
        }
    }
}

pub fn rngsum(start: i32, stop: i32) -> Result<i32, CalculationError> {
    if start > stop {
        Err(CalculationError::InvalidRange)
    } else if start == stop {
        Err(CalculationError::InvalidSpacing)
    } else {
        let mut total = 0;
        for n in start..=stop {
            total += n;
        }
        Ok(total)
    }
}

#[derive(Debug,PartialEq)]
pub enum CalculationError {
    InvalidRange,
    InvalidSpacing,
}
