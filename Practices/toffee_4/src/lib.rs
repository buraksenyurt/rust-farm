#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armstrong_numbers_test() {
        assert_eq!(are_you_armstrong_number(9), true);
        assert_eq!(are_you_armstrong_number(10), false);
        assert_eq!(are_you_armstrong_number(153), true);
        assert_eq!(are_you_armstrong_number(154), false);
        assert_eq!(are_you_armstrong_number(371), true);
        assert_eq!(are_you_armstrong_number(999), false);
    }

    #[test]
    fn square_diff_with_for_test() {
        let result = square_diff_with_for(10);
        assert_eq!(result, 2640);
    }

    #[test]
    fn square_diff_with_formula_test() {
        let result = square_diff_with_formula(10);
        assert_eq!(result, 2640);
    }

    #[test]
    fn square_diff_with_hof_test() {
        let result = square_diff_with_hof(10);
        assert_eq!(result, 2640);
    }

    #[test]
    fn collatz_numbers_test() {
        let number = 12;
        assert_eq!(find_collatz_bad(number), 9);
        assert_eq!(find_collatz_bad(23), 15);
    }

    #[test]
    fn collatz_number_with_match_test() {
        assert_eq!(find_collatz_with_match(12, 0), Some(9));
        assert_eq!(find_collatz_with_match(23, 0), Some(15));
        assert_eq!(find_collatz_with_match(0, 0), None);
        assert_eq!(find_collatz_with_match(1, 0), Some(0));
    }

    #[test]
    #[should_panic]
    fn find_collatz_with_iterator_count_test() {
        assert_eq!(find_collatz(12), Ok(9));
        assert_eq!(find_collatz(23), Ok(15));
        assert_eq!(find_collatz(0), Err("Sıfır Zaten"));
        assert_eq!(find_collatz(1), Ok(0));
    }
}

pub fn are_you_armstrong_number(number: u32) -> bool {
    // digits_count'u HOF içerisine dahil etmenin bir yolu olabilir mi?
    let digits_count = number.to_string().chars().count() as u32;

    // Tüm işi aslında Higher Order Function'lar ile çözümleyebiliriz

    // sayının rakamlarını map ile 10luk düzende alır ve u32 türlü bir vector'e koyar (map)
    // her bir rakamın sayıdaki rakam sayısı kadar üssünü bulur (ikinci map)
    // bunların toplamını çeker (sum)
    let sum_of: u32 = number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        //.collect::<Vec<u32>>()
        //.iter()
        .map(|n| n.pow(digits_count))
        .sum();

    // işlemler sonucu sayıya eşit mi değil mi? Eşitse Armstrong sayısıdır.
    sum_of == number
}

// 0'dan N'e kadar olan sayıların toplamının karesi ile,
// aynı aralıktaki sayıların karelerinin toplamı arasındaki farkı,
// for döngülerini kullanarak bulan fonksiyon örneği.
pub fn square_diff_with_for(max: u32) -> u32 {
    let mut total = 0;
    let mut sum2 = 0;
    for n in 0..=max {
        total += n;
        sum2 += n.pow(2);
    }
    let sum1 = total.pow(2);
    sum1 - sum2
}

// Kareler farkı problemini bu kez matematik formüllerini kullanarak buluyoruz
pub fn square_diff_with_formula(n: u32) -> u32 {
    let sum1 = (n * (n + 1) * (2 * n + 1)) / 6;
    let sum2 = (n * (n + 1) / 2).pow(2);
    sum2 - sum1
}

// Kareler farkı problemini bu kez HOFs'lardan yararlanarak buluyoruz
pub fn square_diff_with_hof(max: u32) -> u32 {
    let sum1 = (0..=max).sum::<u32>().pow(2);
    let sum2 = (0..=max).map(|n| n.pow(2)).sum::<u32>();
    sum1 - sum2
}

// Parametre olarak gelen sayıyı baz alarak collatz serisini çıkaracağız ve
// kaç hamlede 1'e ulaştığımızı döndüreceğiz

// Pekte şık olmayan klasik çözüm
pub fn find_collatz_bad(mut n: u32) -> u32 {
    let mut counter = 0;
    for _ in 0.. {
        if n == 1 {
            break;
        } else {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = (n * 3) + 1;
            }
            counter += 1;
        }
    }
    counter
}

// recursive ve pattern matching kullanılan versiyon. for döngüsü yok dikkat edileceği üzere
pub fn find_collatz_with_match(n: u32, counter: u32) -> Option<u32> {
    // İlk bacakta n 0'sa zaten hesaplama olamaz, None döner.
    // İkinci bacakta 1 sayısına ulaşılmış demektir ve counter adedi dönülebilir.
    // Üçüncü bacağa girilmişse 2 ile bölümden kalan 0 demektir. sayı ikiye bölünür ve yeniden fonksiyon çağırılır.
    // Dördünce bacakta ise üstteki koşullar dışındaki durum söz konusudur. Sayı tektir. 3 ile çarpılıp 1 artırılır ve yeniden fonksiyon çağırılır.
    match (n, n % 2) {
        (0, _) => None,
        (1, _) => Some(counter),
        (i, 0) => find_collatz_with_match(i / 2, counter + 1),
        (i, _) => find_collatz_with_match((i * 3) + 1, counter + 1),
    }
}

// Birde iteratif çözümler var. Collatz sayısının kendisini bir veri yapısı gibi düşünüp
// ona Iterator trait'ini implemente ederek gidilen çözümler de var.
pub struct CollatzNumber {
    number: usize,
}

// CollatzNumber'a iterator trait'ini uyguluyoruz.
// Yapılan bir sonraki elemanı hesaplamayı kolaylaştıracak bir iterasyon oluşturmak
// Bu sayede Count gibi bir fonksiyonu kullanabiliriz.
impl Iterator for CollatzNumber {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.number {
            0 | 1 => None,
            n => {
                match n % 2 {
                    0 => self.number = n / 2,
                    _ => self.number = (3 * n) + 1,
                };
                Some(self.number)
            }
        }
    }
}

pub fn find_collatz(n: usize) -> Result<usize, &'static str> {
    match n {
        0 => Err("Sıfır zaten"),
        _ => Ok(CollatzNumber { number: n }.count()),
    }
}
