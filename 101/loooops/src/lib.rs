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

    #[test]
    fn char_counts_test() {
        let words = vec!["siyah", "beyaz", "en", "büyük", "beşiktaş", "."];
        let result = char_counts(&words);
        assert_eq!(result[0], ("siyah", 5));
        assert_eq!(result[1], ("beyaz", 5));
        assert_eq!(result[2], ("en", 2));
        assert_eq!(result[3], ("büyük", 5));
        assert_eq!(result[4], ("beşiktaş", 8));
        assert_eq!(result[5], (".", 1));
    }
}

// İki değer aralığındaki sayıların toplamını bulan fonksiyon
// Geriye Result dönüyoruz. Eğer kural ihlali varsa CalculationError dönmekte
pub fn rngsum(start: i32, stop: i32) -> Result<i32, CalculationError> {
    // start, stop ikilisini kıyasladığımız matching
    match (start, stop) {
        (s, e) if s > e => Err(CalculationError::InvalidRange),
        (s, e) if s == e => Err(CalculationError::InvalidSpacing),
        (s, e) => {
            let mut total = 0;
            // basit olarak s değerine e değerine kadar giden bir döngü
            for n in s..=e {
                total += n;
            }
            Ok(total)
        }
    }
    /*    if start > stop {
        Err(CalculationError::InvalidRange)
    } else if start == stop {
        Err(CalculationError::InvalidSpacing)
    } else {
        let mut total = 0;
        for n in start..=stop {
            total += n;
        }
        Ok(total)
    }*/
}

// String elemanlar içeren bir vektördeki her bir ifadede kaç harf olduğunu dönen kobay fonksiyon
// yaşam sürelerini belirttiğimiz 'a olayına 101 eğitimi için çok takılmayalım.
pub fn char_counts<'a>(words: &'a [&str]) -> Vec<(&'a str, usize)> {
    let mut results = Vec::<(&str, usize)>::new();
    // iterator ile kullanılan bir for döngüsü
    for word in words.iter() {
        let l = word.chars().count();
        results.push((word, l));
    }
    results
}

#[derive(Debug, PartialEq)]
pub enum CalculationError {
    InvalidRange,
    InvalidSpacing,
}
