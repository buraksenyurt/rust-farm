#[cfg(test)]
mod tests {
    use crate::{get_rnd_product, get_string};

    #[test]
    fn create_tuple_test() {
        // Farklı veri türlerinin bir araya geldiği bir veri yapısını tuple olarak tanımlayabiliriz.
        let player = ("veyn runi", 10, "İngiltere", false);

        // Tuple elemanlarına indis numaraları ile aşağıdaki gibi erişebiliriz.
        assert_eq!(player.0, "veyn runi");
        assert_eq!(player.1, 10);
        assert_eq!(player.2, "İngiltere");
        assert_eq!(player.3, false);

        // Dilersek tuple alanlarının veri türlerini açıkça belirtebiliriz.
        let player: (&str, u8, bool, f32) = ("karamela entıni", 11, true, 1_000_000.50);
        assert_eq!(player.0, "karamela entıni");
        assert_eq!((player.1, player.2), (11, true));
    }

    #[test]
    fn deconstructing_tuple_test() {
        let game = ("pro evoluyşın soccer II", 59.99, 100, "caddebostan");
        // Tuple alanlarını deconstructing ile isim vererek kullanabiliriz
        let (name, price, stock_level, shop) = game;
        assert_eq!(name, "pro evoluyşın soccer II");
        assert_eq!(price, 59.99);
        assert_eq!((stock_level, shop), (100, "caddebostan"));
    }

    #[test]
    fn change_tuple_items_test() {
        let mut game = ("pro evoluyşın soccer II", 59.99, 100, "caddebostan");
        game.0 = "Pro Evolution Soccer 2022";
        assert_eq!(game.0, "Pro Evolution Soccer 2022");
    }

    #[test]
    fn return_tuple_from_a_fn_test() {
        let model = get_rnd_product();
        assert_eq!(model.0, 1001);
        let (_, name, price, _) = get_rnd_product();
        assert_eq!(name, "Revell Spitfire 1:72 Maket");
        assert_eq!(price, 250.0);
    }

    #[test]
    fn argument_tuple_test() {
        let data = (1, "Initial State", 23.30);
        assert_eq!(get_string(data), "1-`Initial State`.(23.3)");
    }
}

// Pek tabii fonksiyonlardan geriye Tuple döndürebiliriz.
// literal string'in yaşam ömrünü derleyici bilemeyeceğinden &'static olarak tanımlanmıştır.
pub fn get_rnd_product() -> (i32, &'static str, f32, bool) {
    (1001, "Revell Spitfire 1:72 Maket", 250.0, true)
}

pub fn get_string(item: (i32, &str, f32)) -> String {
    format!("{}-`{}`.({})", item.0, item.1, item.2)
}
