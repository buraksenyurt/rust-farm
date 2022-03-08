/// Kullanıcının girdiği kelime üstünde bazı düzeltmeler yapan fonksiyon
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
