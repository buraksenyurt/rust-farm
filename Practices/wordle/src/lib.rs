/*
   Kelime listesi sadece okuma amaçlı kullanılacak.
   include_str! makrosu parametre olarak gelen dosyayı derleme zamanında alıp kaynak kodun içerisine gömer.
   Dolayısıyla data dosyasını release aldıktan sonra programı götürdüğümüz yere taşımaya gerek yoktur.
*/
use bracket_random::prelude::RandomNumberGenerator;
use colored::*;
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
    pub fn new() -> Self {
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
    /*
       Oyun sahamız terminal ekranı. Manager'ın tuttuğu kelime ve
       oyuncunun tahminlerine göre 5X6 lık matrisi çizen bir fonksiona ihtiyacımız var.

       self üzerinden guessed_letters vector'üne oyuncunun tahmin ettiği
       ama programın tuttuğu kelimede olmayan harfler eklenecek.
       Bu nedenle self, mutable referans olarak alındı.
    */
    /// Oyun sahasını tahmin edilen kelimeler ve sonuçları ile çizer
    pub fn draw_board(&mut self) {
        // önce yapılan tahminleri gezen bir döngü açıyoruz.
        // for_each fonksiyonunda bir tuple kullandığımıza dikkat edelim.
        // Bu tuple'da satır numarası ve guesses vector'ündeki kelime yer alıyor.
        // Yani guesses vector elemanlarını dolaşırken indislerini de satır numarası olarak kullanabiliriz.
        self.guesses
            .iter()
            .enumerate()
            .for_each(|(row_index, guess)| {
                // Şimdi bulunduğumuz satırdaki kelimenin harflerini dolaşacağız
                // Yine for_each döngüsü kullanılıyor. Her iterasyonda kelimedeki karakteri ve indisini bir tuple ile ele alıyoruz.
                guess.chars().enumerate().for_each(|(i, c)| {
                    // Şimdi karakterleri programın tuttuğu kelimedekiler ile karşılaştıracağız.

                    // Eğer chosen_word'deki i. sıradaki karakter guess'teki c karakterine eşitse
                    // harf doğrudur ve kelimede doğru yerdedir
                    let row = if self.chosen_word.chars().nth(i).unwrap() == c {
                        format!("{}", c).bright_green()
                    } else if self.chosen_word.chars().any(|wc| wc == c) {
                        // Harf doğrudur ama yeri yanlıştır. Bunu da any fonksiyonu üstünden kontrol edebiliriz.
                        format!("{}", c).bright_yellow()
                    } else {
                        // Harf programın tuttuğu kelimede yoksa bu durumda tahmin edilen harfler
                        // listesine eklenir ve kullanıcının karakteri kırmızıya boyanır.
                        self.guessed_letters.insert(c);
                        format!("{}", c).red()
                    };
                    print!("{}", row);
                });
                println!(); // Bir alt satıra geç
            })
    }

    /*
       Wordle oyununda kullanılan harflerde gösterilmekte.
       Bunun için de yardımcı bir fonksiyon kullanılabilir
    */
    /// Oyuncunun kullandığı ama programın tuttuğu kelimede olmayan harflerin listesini ekrana basar.
    pub fn show_invalid_letters(&self) {
        if !self.guessed_letters.is_empty() {
            println!("Bu harfleri kullandın.\nAncak tahmin ettiğimi kelimede yoklar.".purple());
            self.guessed_letters.iter().for_each(|c| print!("{}", c));
            println!()
        }
    }

    /*
       Bu fonksiyon kullanıcıdan tahminini alıp kontrol etmekte.
       Geçerli bir uzunlukta mı, oyunun kullandığı sözlük içerisinde yer alıyor mu gibi.
    */
    /// Oyuncudan tahminini alır ve belli kurallara göre kontrol eder
    pub fn take_guess(&mut self) -> String {
        println!(
            "Hey! Oyuncu.\nHadi bana {} karakterden oluşan bir kelime yaz ve ENTER'a bas".purple(),
            WORD_LENGTH
        );
        // Önce tahminde olupta programın tuttuğu kelimede olmayan harfleri gösterelim
        self.show_invalid_letters();
        // Kullanıcının tahmini ve kelimenin geçerli olup olmadığını tutan iki mutable değişkenimiz var.
        let mut user_guess = String::new();
        let mut is_guess_valid = false;
        // Döngümüz kullanıcının girdiği kelime tüm kuralları sağlayana kadar devam edecek
        while !is_guess_valid {
            user_guess = String::new();
            // Oyuncunun girdisi terminal ekranından olacak.
            // İçeriği read_line fonksiyonu ile user_guess değişkenine yazabiliriz.
            // Çok büyük bir problem olmayacağını düşünerekten unwrap ile girilen bilgiyi alıyoruz.
            std::io::stdin().read_line(&mut user_guess).unwrap();
            // Kelimedeki gereksiz boşlukları çıkartıp harfleri büyük harfe çeviren bir fonksiyonumuz var.
            // Kelimeyi o işlemden geçiriyoruz.
            user_guess = sanitize_word(&user_guess);
            // Kontrollerimiz basit. Kelime belirlediğimiz uzunlukta olmalı ve
            // programın kullandığı sözlükte yer almalı.
            // Eğer böyle değilse is_guess_valid değişkeni false olarak kalacak ve döngü devam edecek
            if user_guess.len() != WORD_LENGTH {
                println!(
                    "{}",
                    format!("{} uzunluğunda bir kelime girmelisin.", WORD_LENGTH).red()
                )
            } else if !self.dictionary.iter().any(|word| word == &user_guess) {
                println!("{}", "Girdiğin kelime benim dükkanda bile yok :S".red())
            } else {
                self.guesses.push(guess.clone());
                is_guess_valid = true;
            }
        }
        // Kod buraya geldiyse geçerli bir tahmin elimizdedir. Bunu fonksiyondan geri dönüyoruz.
        user_guess
    }
}

#[cfg(test)]
mod test {
    use crate::{sanitize_word, word_list, Manager};

    #[test]
    fn should_manager_crated_successfully() {
        let poe = Manager::new();
        assert_eq!(poe.chosen_word.chars().count(), 5);
        assert!(poe.available_words.len() > 0);
        assert!(poe.guesses.len() == 0);
    }

    #[test]
    fn should_sanitize_word_fn_works() {
        let word = "gol Dy   ";
        let result = sanitize_word(word);
        assert_eq!(result, "GOLDY");
    }

    #[test]
    fn should_world_list_fn_works() {
        let words = word_list();
        assert!(words.len() > 1);
        let count = words.iter().filter(|w| w.chars().count() != 5).count();
        assert_eq!(count, 0);
    }
}
