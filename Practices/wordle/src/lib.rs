/*
   Kelime listesi sadece okuma amaçlı kullanılacak.
   include_str! makrosu parametre olarak gelen dosyayı derleme zamanında alıp kaynak kodun içerisine gömer.
   Dolayısıyla data dosyasını release aldıktan sonra programı götürdüğümüz yere taşımaya gerek yoktur.
*/
use bracket_random::prelude::RandomNumberGenerator;
use std::collections::HashSet;

const WORDS: &str = include_str!("words.data");
const WORD_LENGTH: usize = 5; // Kelime maksimum 5 harfli olabilir
const TRY_COUNT: usize = 6; // Oyuncuya 6 deneme hakkı veriyoruz

/// Kelime üstünde bazı iyileştirmeler yapan fonksiyondur.
fn sanitize_word(word: &str) -> String {
    /*
    Bir kaç Higher Order Function kullanarak gelen kelime üstünde işlmeler yapılmakta.
    önce gereksiz boşluklar trim ile atılıyor.
    Kelime büyük harfe çevriliyor ve chars fonksiyonu ile tüm karakter listesi alınıyor.
    Rust'ta tüm string'ler UTF-8 formatında. Dolayısıyla aralara harf olmayan karakterler(emoji gibi) gelebilir.
    Bunu önlemek için kelimede yer alan ascii karakterler bulunuyor. filter fonksiyonu bunun için kullanılmakta.
    Son olarak bulunan karakterler collect ile toplanıp fonksiyondan String değişken olarak döndürülüyor.
     */
    word.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect()
}

/// Kelimeleri String türden vektöre alan fonksiyon
fn word_list() -> Vec<String> {
    /*
    Şimdi elimizde kelimeleri tutan dosya var. Bunu WORDS isimli constant'ta tutuyoruz.
    Bu dosyadaki herbir satırı okuyup, sanitize işleminden geçirdikten sonra,
    uzunluğu 5 karakter olanları String türden bir vector'de topluyoruz.
     */
    WORDS
        .split('\n')
        .map(sanitize_word)
        .filter(|line| line.len() == WORD_LENGTH)
        .collect()
}

/// Yönetici sınıf. Kelimeler, seçilen kelimeyi, tahmin edilen harfleri ve tahmin edilen kelimeleri yönetir
struct Manager {
    available_words: Vec<String>,
    chosen_word: String,
    guessed_letters: HashSet<char>,
    guesses: Vec<String>,
}

impl Manager {
    /*
       Yapıcı metot kelimelerin olduğu sözlükten rastgele bir kelimeyi de seçerek bir Manager örneği döner.
    */
    fn new() -> Self {
        // Rastgele sayı üretici
        let mut rnd_gnr = RandomNumberGenerator::new();
        let dictionary = word_list();
        // random_slice_entry fonksiyonu parametre olarak gelen dilimden rastgele bir tane çeker.
        // değerin bir klonunu word değişkenine alırız.
        let chosen_word = rnd_gnr.random_slice_entry(&dictionary).unwrap().clone();
        Self {
            available_words: dictionary,
            chosen_word,
            guessed_letters: HashSet::new(),
            guesses: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Manager;

    #[test]
    fn should_manager_crated_successfully() {
        let poe = Manager::new();
        assert_eq!(poe.chosen_word.chars().count(), 5);
        assert!(poe.available_words.len() > 0);
        assert!(poe.guesses.len() == 0);
    }
}
