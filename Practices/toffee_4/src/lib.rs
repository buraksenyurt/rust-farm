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
