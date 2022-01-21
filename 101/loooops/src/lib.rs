use rand::Rng;

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
        let expected = vec![
            ("siyah", 5),
            ("beyaz", 5),
            ("en", 2),
            ("büyük", 5),
            ("beşiktaş", 8),
            (".", 1),
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn perfect_divisors_test() {
        let result = get_perfect_divisors(21, 7);
        let expected = vec![7, 14, 21];
        assert_eq!(expected, result);
    }

    #[test]
    fn get_random_vec_test() {
        let result = get_random_vec(10);
        assert_eq!(result.len(), 10);

        let mut i = 0;
        for (pos, n) in result.iter().enumerate() {
            assert!(pos == i);
            assert!(*n > 0);
            i += 1;
        }
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

// Maks değerine kadarki sayılarda ikinci parametreye tam bölünenlerin listesini döner
pub fn get_perfect_divisors(max: i32, div: i32) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    let mut i = 0;
    // while döngüsü koşul sağlandığı sürece devam eder
    while i <= max {
        i += 1;
        if i % div == 0 {
            result.push(i);
        }
    }
    result
}

// Belirtilen uzunlukta bir vecktör oluşturup içerisini rastgele sayılarla dolduran kobay fonksiyon.
// Bu seferkinde loop döngüsü söz konusu.
pub fn get_random_vec(length: i32) -> Vec<i32> {
    let mut i = 0;
    let mut numbers = Vec::<i32>::new();
    let mut rng = rand::thread_rng();
    // loop aslında while gibi sonsuz bir döngü bloğu açar.
    loop {
        // Sembolik bir iç döngü...
        // Üretilen rastgele sayının 0'dan farklı olmasını istediğimiz için
        // , sıfırdan farklı bir tane ele edene kadar deniyoruz. Daha kolay yolu da var tabii
        // ama sırf iç döngü örneği olsun diye :)
        loop {
            let n = rng.gen();
            if n <= 0 {
                continue;
            } else {
                numbers.push(n);
                break;
            }
        }
        i += 1;
        if i == length {
            break; // sonsuz döngüyü kırdığımız satır
        }
    }
    numbers
}

#[derive(Debug, PartialEq)]
pub enum CalculationError {
    InvalidRange,
    InvalidSpacing,
}
