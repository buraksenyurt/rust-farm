use rand::Rng;
use std::io;
use std::process::exit;

/*
 Örnek oldukça basit. 1 ile 10 arasında rastgele sayı üretiliyor.
 Kullanıcıdan bunu tahmin etmesi isteniyor ve üç hak veriliyor.
*/
fn main() {
    // rastgele sayıyı alıyoruz
    let number = get_number();
    println!("Aklımdan 1-10 arasında bir sayı tuttum.\nBakalım bilebilecek misin?\n3 Hakkın var.");
    let try_count = 3;
    let mut counter = 0;
    // Sonsuz bir döngü lakin counter'a göre hareket ediyor. 3 hakkı kontrol ediyor.
    loop {
        counter += 1;

        // Terminal girdisini önce bir String değişkene alacağız
        let mut user_input = String::new();
        // Olası bir hatada panik mesajına ilave de yapıyoruz
        io::stdin()
            .read_line(&mut user_input)
            .expect("Okuma sırasında hata");
        // Girilen içeriği parse ile u8 türüne dönüştürüyoruz.
        // trim çağrısı önemli. Kullanmazsak sondaki fazladan karakter nedeniyle hata alırız
        let guess = user_input
            .trim()
            .parse::<u8>()
            .expect("Tür dönüştürme hatası");

        // Sayıyı doğru tahmin ettiysek tebrik edip çıkıyoruz
        if guess == number {
            println!("Bravo bildin");
            exit(1);
        } else {
            if counter == try_count {
                println!(
                    "Üzgünüm ama tüm hakların tükendi :( Aklımdaki sayı şu : {}",
                    number
                );
                // programdan çıkış için
                exit(1);
            }
            // doğru tahmin edemediysek continue ile döngüyü devam ettiriyoruz.
            // Tabii döngü hakkımız dolana kadar devam ediyor.
            println!("Bir daha dene!");
            continue;
        }
    }
}

fn get_number() -> u8 {
    let mut rnd = rand::thread_rng();
    rnd.gen_range(1..=10)
}
