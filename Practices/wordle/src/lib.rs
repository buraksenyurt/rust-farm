/*
   Kelime listesi sadece okuma amaçlı kullanılacak.
   include_str! makrosu parametre olarak gelen dosyayı derleme zamanında alıp kaynak kodun içerisine gömer.
   Dolayısıyla data dosyasını release aldıktan sonra programı götürdüğümüz yere taşımaya gerek yoktur.
*/
const WORDS: &str = include_str!("words.data");
const WORD_LENGTH: usize = 5;

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
