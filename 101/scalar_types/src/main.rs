// Bir sabit tanımı. Sabitlerde veri tipi açıkça belirtilmelidir.
// Sabitler sabittir :) Mutable tanımlanamazlar.
// local veya global scope boyunca yaşarlar
const DEFAULT_LEVEL: u8 = 70;

fn main() {
    let lucky_num = 23; // Varsayılan olarak 32bit integer (i32)
    let player_name = "Baba The Fat"; // string literal (&str)
    println!("Oyuncu adım {} ve Şans numaram {}",player_name,lucky_num);

    // let max_limit=12345678900; //the literal `12345678900` does not fit into the type `i32` whose range is `-2147483648..=2147483647`

    // Kullanılmayan değişkenlerin başına _ ekleyerek uyarıları atlatabiliriz. Ya da #[allow(unused_variables)] ile.
    let _max_limit: i64 = 12345678900; // 64 bit tamsayı
}
