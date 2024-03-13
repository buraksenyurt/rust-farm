/*
get_max_value fonksiyon gelen i32 türünden dizideki en büyük elemanı bulurken divide and conquer
algoritmasını baz alır. Dizi önce orta noktadan ikiye bölünür. Bu bölünme operasyonu
dizinin 1 elemanı kalana kadar da devam eder. Yani recursive(Özyinelemeli) çağrılar söz konusudur.
Her parçalama da maksimum değer bulunur.

Bu fonksiyon recursive çağrılarda da bir dizi alır ve dizinin tüm elemanlarını dolaşır.
Buna göre O(n) türünden bir time complexity değeri vardır diyebiliriz.
*/

use std::collections::HashMap;

pub fn get_max_value(values: &[i32]) -> i32 {
    if values.len() == 1 {
        values[0]
    } else {
        let mid = values.len() / 2;
        let first_max = get_max_value(&values[..mid]);
        let second_max = get_max_value(&values[mid..]);
        if first_max > second_max {
            first_max
        } else {
            second_max
        }
    }
}

/*

    Bu ikinci örnekte bir kelime havuzundaki kelimelerden kaçar adet bulunduğunun hesaplanması için
    divide and conquer algoritmasından yararlanılmaktadır. Burada Map Reduce şeklinde ilerlenilmiştir.
    Öncek her bir kelime için key-value çifti oluşturulur ve
    her kelime için value değeri 1 olarak set edilir. Bu Map aşaması olarak ifade edilir.
    Map aşamasında her bir kelime için anahtar değer çifti oluşturulur ama bu sabit zamanlı bir işlemdir.
    Dolayısıyla Big O değeri O(n) dir.

    Reduce aşamasında ise Map aşamasında elde edilen veri seti üstünde
    ileri yönlü bir iterasyon başlatılır ve her bir kelimenin toplam değeri hesaplanır.
    Reduce fonksiyonu her bir anahtar bilgisi için işlem yapar. Big O değeri O(n) dir.

    get_words_count fonksiyonu alt fonksiyon çağrılarına göre O(2n) değerinde zaman karmaşıklığına
    sahiptir. O(n) + O(n) olması sebebiyle. Ancak Big O notasyonunda sabit katsayılar önemsenmediğinden
    Time Complexity(Zaman Karmaşıklığı) değeri O(n) olarak ifade edilir.
 */
pub struct WordProcessor;

impl WordProcessor {
    pub fn get_words_count(words: Vec<&str>) -> HashMap<&str, i32> {
        let mapped = Self::map(words);
        Self::reduce(mapped)
    }

    fn map(words: Vec<&str>) -> Vec<(&str, i32)> {
        words.into_iter().map(|w| (w, 1)).collect()
    }

    fn reduce(mapped: Vec<(&str, i32)>) -> HashMap<&str, i32> {
        let mut counted = HashMap::new();
        for (w, c) in mapped {
            *counted.entry(w).or_insert(0) += c;
        }
        counted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_max_value_returns_valid_value_test() {
        let numbers = vec![5, 6, 1, 9, 48, 10, 2, 8, 2, 7, 7, 10, 32, 23, 0, 6, 21, 22];
        let actual = get_max_value(&numbers);
        let expected = 48;
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_words_count_returns_valid_counts_test() {
        let words = vec![
            "for", "for", "if", "then", "if", "else", "select", "join", "where", "select",
            "select", "equal", "for",
        ];
        let actual = WordProcessor::get_words_count(words);
        let mut counted = HashMap::new();
        counted.insert("for", 3);
        counted.insert("if", 2);
        counted.insert("then", 1);
        counted.insert("else", 1);
        counted.insert("select", 3);
        counted.insert("join", 1);
        counted.insert("where", 1);
        counted.insert("equal", 1);
        assert_eq!(actual, counted);
    }
}
