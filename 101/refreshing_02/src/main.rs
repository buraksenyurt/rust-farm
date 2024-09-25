/*
    Fonksiyonlardan Result<T,E> türünü döndürme meselesini bir hatırlayalım.

    Eğer beklenmedik (unexpected) bir hata söz konusu ise panic kullanımı makuldur.
    Ancak kestirebildiğimiz bir hata olası ise Result<> kullanmak gerekir.
    Örneğin aşağıdaki fonksiyon da paydanın 0 gelmesi olasıdır. Bu beklenen türden bir hatadır.
    Bu nedenle geriye Result ile fonksiyonun başarılı olup olmadığını, başarılı ise de elde edilen
    değeri döndürmek mantıklıdır.
*/

fn main() {
    if let Ok(result) = div(3.14, 0.0) {
        println!("İşlem sonucu {}", result);
    } else {
        println!("İşlem sonucu alınamadı");
    }

    match div(3.14, 0.0) {
        Ok(result) => {
            println!("İşlem sonucu {}", result)
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }

    match is_valid_url("blabla.com".to_string()) {
        Ok(_) => println!("Continue"),
        Err(e) => println!("{:?}", e),
    }
    match is_valid_url("https://www.buraksenyurt.com".to_string()) {
        Ok(_) => println!("Continue"),
        Err(e) => println!("{:?}", e),
    }
}

fn div(x: f32, y: f32) -> Result<f32, Problem> {
    if y == 0f32 {
        // Generic Error döndürebileceğimiz gibi kendi Err nesnemizi de döndürebiliriz
        Err(Problem::DivideByZero)
    } else {
        Ok(x / y)
    }
}

/*
   Bazı hallerde de bir şeyleri kontrol eder ve hata olup olmadığını döneriz.
   Söz gelimi metin tabanlı bir içeriğin geçerli bir web adresi olup olmadığını kontrol
   etmek için aşağıdaki fonksiyonu ele alabiliriz.
   Bu durumda geriye değer döndürmek yerine boş tuple yani Ok(()) dönülebilir.
   Tabii bu bir zorunluluk değil sadece bir gelenektir(Convention)
*/
fn is_valid_url(url: String) -> Result<(), Problem> {
    // Sembolik olarak starts_with'e bakıyoruz. Elbette bir regex çalıştırmak daha doğru olur
    if url.starts_with("https://") {
        Ok(())
    } else {
        Err(Problem::InvalidUrl)
    }
}

#[derive(Debug)]
enum Problem {
    DivideByZero,
    InvalidUrl,
}