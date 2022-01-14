#[cfg(test)]
mod tests {
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
}
