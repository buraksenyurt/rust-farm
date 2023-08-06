use crate::common::*;
use crate::game::Game;
use crate::http_request::HttpRequestBuilder;
use crate::web_request::WebRequestBuilder;

mod common;
mod game;
mod http_request;
mod web_request;

fn main() -> Result<()> {
    /*
       1,2,3 ve 4 numaralı örnekler basit alanlar içeren kolak oluşturulabilir nesneler içindir.
       Daha karmaşık nesnelerin oluşturulmasında builder pattern'lerden yararlanılabilir.
    */
    // #1 Bir nesneyi oluşturmanın basit yolu
    let game = Game {
        title: "Prince of Persia".to_string(),
        released: false,
        summary: None,
    };
    println!("{game:#?}");

    // #2 bir constructor fonksiyon yardımıyla nesne oluşturmak
    let game = Game::new("Command & Conquer Red Alert II");
    println!("{game:#?}");

    // #3 Bir başka nesne oluşturma yolu da Default trait'den implemente etmektir.
    let game = Game::default();
    println!("{game:#?}");
    // Pek çok fonksiyon Default trait implementasyonuna bakar. Örneğin,
    let game: Option<Game> = None; // None olarak gelen bir Game değişkeni
    let game = game.unwrap_or_default(); // None olduğu için Default fonksiyonu çağrılır
    println!("{game:#?}");

    // Default trait'i aşağıdaki gibi kullanımları da mümkün kılar
    let game = Game {
        released: true,
        ..Default::default()
    };
    println!("{game:#?}");

    // #5 Bir nesneyi builder fonksiyonu ile oluşturmak
    // Aşağıdaki örneklerde Non-Consuming Builder Pattern uyarlaması söz konusudur.
    let request = WebRequestBuilder::new()
        .url("https://localhost:5679/api/games")
        .method("POST")
        .header("content-type", "application/json")
        .body("{'name':'Barbarian'}")
        .build()?;
    println!("{request:#?}");

    // Builder deseninden yola çıkarak bir builder kullanan n sayıda nesne de kullanabiliriz
    // Yani tek bir WebRequestBuilder oluşturup n sayıda WebRequest nesnesi söz konusu olabilir.
    let mut request_builder = WebRequestBuilder::new();
    request_builder
        .url("https://localhost:5679/api/games")
        .method("GET");
    let request = request_builder
        .header("token", "bearer 23929329")
        .header("content-type", "application/json")
        .build()?;
    println!("{request:#?}");

    request_builder
        .method("POST")
        .body("{'name':'Barbarian','release':true,'summary':'Eski bir C64 oyunu}");
    let request = request_builder.build()?;
    println!("{request:#?}");

    // #6 Bu sefer &mut self yerine Ownership kurallarına daha çok uyan
    // Consuming Builder Pattern uyarlaması söz konusudur.
    // Örneklerin karışmaması için http_request şeklinde bir veri yapısı ele alınmıştır.
    let http_builder = HttpRequestBuilder::new()
        .url("https://localhost:5679/api/games")
        .method("GET")
        .header("token", "bearer 239ERE329043")
        .header("content-type", "application/json");
    let request = http_builder.build()?;
    println!("{request:#?}");

    // Tabii bu durumda build fonksiyonunu aynı builder nesnesinden birçok kez kullanmak istersek,
    // HttpRequestBuilder'ın Clonable olmasına dikkat etmemiz gerekir
    // Bunu görmek için HttpRequestBuilder nesnesindeki Clone trait bildirimini kaldırıp deneyelim.
    /*

        error[E0382]: use of moved value: `http_builder`
      --> src/main.rs:91:24
       |
    84 |     let http_builder = HttpRequestBuilder::new()
       |         ------------ move occurs because `http_builder` has type `HttpRequestBuilder`, which does not implement the `Copy` trait
    ...
    88 |     let request = http_builder.build()?;
       |                                ------- `http_builder` moved due to this method call
    ...
    91 |     let http_builder = http_builder
       |                        ^^^^^^^^^^^^ value used here after move
       |
    note: `HttpRequestBuilder::build` takes ownership of the receiver `self`, which moves `http_builder`
      --> src/http_request.rs:46:18
       |
    46 |     pub fn build(self) -> Result<HttpRequest> {
       |

         */

    let http_builder = HttpRequestBuilder::new()
        .url("https://localhost:5679/api/games")
        .header("token", "bearer 239ERE329043")
        .header("content-type", "application/json");
    let request = http_builder.clone().build()?; // Derive Clone varsa clone işe yarar
    println!("{request:#?}");

    let http_builder = http_builder
        .method("POST")
        .body("{'name':'Barbarian','release':true,'summary':'Eski bir C64 oyunu}");
    let request = http_builder.build()?;
    println!("{request:#?}");

    Ok(())
}
