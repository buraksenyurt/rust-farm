use std::f32::consts::PI; // sistemdeki PI sabiti

// Bir sabit tanımı. Sabitlerde veri tipi açıkça belirtilmelidir.
// Sabitler sabittir :) Mutable tanımlanamazlar.
// local veya global scope boyunca yaşarlar
const DEFAULT_LEVEL: u8 = 70;

fn main() {
    println!("Oyuna giriş için varsayılan seviye {}", DEFAULT_LEVEL);

    let lucky_num = 23; // Varsayılan olarak 32bit integer (i32)
    let player_name = "Baba The Fat"; // string literal (&str)
    println!(
        "Oyuncu adım `{}` ve Şans numaram {}.",
        player_name, lucky_num
    );

    // let max_limit=12345678900; //the literal `12345678900` does not fit into the type `i32` whose range is `-2147483648..=2147483647`

    // Kullanılmayan değişkenlerin başına _ ekleyerek uyarıları atlatabiliriz. Ya da #[allow(unused_variables)] ile.
    let _max_limit: i64 = 12345678900; // 64 bit tamsayı

    // Tüm değişkenler varsayılan olarak immutable'dır.
    // let width=10;
    // width+=1; //Cannot assign twice to immutable variable

    let mut distance = 3.2; // varsayılan olarak 64 bit float
    distance += 1.0;
    println!("Gezegene olan mesafe {} ışık yılı.", distance);

    // let distance:f32=3; //mismatched types hatası. Atama 3.0 veya 3_f32 şeklinde olmalı.
    let _distance: f32 = 3_f32;

    let error_code = 404;
    let error_code = "HTTP 404 -> Not Found"; // shadowing
    println!("Hata kodu `{}`", error_code);

    // Birden fazla değişkeni tek seferde atayabiliriz.
    let (id, title, price) = (1000_u16, "Kablosuz Mikrofonlu Kulaklık", 450.99);
    // :? ifadesindeki ?, std::fmt::Debug trait'inin tetiklenmesini sağlar.
    // Bir trait ile belli bir davranışın uygulanmasını sağlarız.
    // Bu örnekte println makrosu, (id,title,price) değişkenin ekrana yazdırılması için varsayılan bir davranış uygular.
    println!("Ürün bilgisi...{:?}", (id, title, price));

    // Büyük sayıların koddaki okunurluğu _ ayracı ile artırılabilir
    let beginner_salary = 1_250_000;
    println!(
        "Yıllık bazda giriş maaşınız {} metaverse coin.",
        beginner_salary
    );
    let pi = 3.141_592_653_589_793;
    println!("Pi sayısı. {}", pi);

    // Aslında PI içim
    println!("Sistemdeki pi sabitinin değeri -> {}",PI);

    let _some_char = 'b'; // char
    let cake = '\u{1F382}'; // Unicode karakter kullanımı
    println!(
        "Doğum günün kutlu olsun. İşte pastan\n{0}\n\t{0}\n\t\t{0}",
        cake
    );

    // static yaşam süresine sahip bir literal string tanımı
    let hero: &'static str = "Ferris";
    println!("Günün kahramanı {}", hero);

    let _book_name = String::new(); // String built-in tasarlanmış bir Struct'tır. new ile yeni nesne örneği oluşturulur.
    let mut chapter_name = String::from("Struct Veri Türünü"); // Bir metin bilgisi ile String nesnesi oluşturmak
    chapter_name.push(' '); // karakter eklemek
    chapter_name.push_str("Anlamak"); // başka bir metin eklemek.
    println!("Bu bölümün adı `{}`", chapter_name);

    let chapter_name = chapter_name.replace(" ", "_"); // metinde değişiklik yapmak
    println!("Bu bölümün adı `{}`", chapter_name);

    // formatlanmış metinsel içerikler hazırlamak için format! makrosu kullanılabilir.
    let intro = format!(
        "Merhaba {}. Sıradaki bölüm {}.\nBaşlamak için bir tuşa bas.",
        player_name, chapter_name
    );
    println!("{}", intro);
}
