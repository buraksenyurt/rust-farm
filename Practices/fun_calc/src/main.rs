use std::io;
use std::process::exit;

fn main() {
    loop {
        println!(
            "Bir şeyin ağırlığını gir.\nKendi kilon, bilgisayarının ki, oyuncak otomobilinin ki\nhatta bir balinanın ki bile olabilir.(kg)"
        );

        // Kullanıcının terminal ekranına girdiği veriyi alacağız.
        // İlk olarak bir String nesne örneklenir ve sonrasında stdin().read_line fonksiyonu ile
        // terminale girilen veri String değişkene yazılır.
        // Bir hata olma durumunda expect ile panic bilgisine bir mesaj ilave ederiz.
        let mut input_weight = String::new();
        io::stdin()
            .read_line(&mut input_weight)
            .expect("Developer hatası :P");

        println!(
            "...ve bir gezegen seç.

        [1] - Merkür
        [2] - Venüs
        [3] - Mars
        [4] - Jüpiter
        [5] - Satürn
        [6] - Uranüs
        [7] - Neptün
        [8] - Ay

        [9] - Uygulamadan Çıkış"
        );

        // Benzer şekilde gezegen seçimi yaptırırız.
        // Üstte yaptığımız gibi read_line ile terminal girdisini alıp planet isimli
        // String değişkene yazmaya çalışırız.
        let mut planet = String::new();
        io::stdin()
            .read_line(&mut planet)
            .expect("Developer hatası :P");

        /*  let match kullanarak seçilen gezegendeki ağırlığı bulmaya çalışıyoruz.
           terminalden girilen bilgi sonunda alt satıra geçme karakteri olacağından trim kullandık.
           parse çağrısı ile girilen değeri f32 türüne çeviriyoruz ki geçerli bir değer girilmezse
           kod akışı Err dalına geçecektir(ParseFloatError)
           Eğer dönüştürülebilen bir değer girilmişse de Ok dalına girecektir.
           Ok dalında dikkat edileceği üzere başka bir match ifadesi yer alır.
           Seçilen gezegen bilgisine göre uygun fonksiyon çağırılır.
           1 ile 8 arasındaki girdiler için uygun fonksiyon çağrısı yapılır.
           9 girilmişse programdan çıkılır.
        */
        let weight: f32 = match input_weight.trim().parse::<f32>() {
            Ok(w) => match planet.trim() {
                "1" => find_weight(w, Planet::Mercury).unwrap(),
                "2" => find_weight(w, Planet::Venus).unwrap(),
                "3" => find_weight(w, Planet::Mars).unwrap(),
                "4" => find_weight(w, Planet::Jupiter).unwrap(),
                "5" => find_weight(w, Planet::Saturn).unwrap(),
                "6" => find_weight(w, Planet::Uranus).unwrap(),
                "7" => find_weight(w, Planet::Neptune).unwrap(),
                "8" => find_weight(w, Planet::Moon).unwrap(),
                "9" => {
                    println!("Görüşmek üzere :)");
                    exit(1)
                }
                _ => {
                    println!("Seçimin yanlış olabilir mi?");
                    0.0
                }
            },
            Err(e) => {
                println!("Sanırım girdiğin kilo sayısal bir değer değil.\n{:?}", e);
                continue;
            }
        };

        println!(
            "Dünyada {} Kg seçtiğin gezegende ise {} kg'sın.\n",
            &input_weight.trim(),
            weight
        );
    }
}

/// Kullanılacak gezegenler
#[allow(dead_code)]
enum Planet {
    Mercury,
    Mars,
    Venus,
    Saturn,
    Neptune,
    Jupiter,
    Uranus,
    Moon,
}

/// Gezegene göre ağırlık bulan fonksiyon
fn find_weight(w: f32, p: Planet) -> Option<f32> {
    match p {
        Planet::Mercury => Some((w / 9.81) * 3.7),
        Planet::Mars => Some((w / 9.81) * 3.711),
        Planet::Venus => Some((w / 9.81) * 8.87),
        Planet::Jupiter => Some((w / 9.81) * 24.79),
        Planet::Neptune => Some((w / 9.81) * 11.15),
        Planet::Uranus => Some((w / 9.81) * 8.69),
        Planet::Saturn => Some((w / 9.81) * 10.44),
        Planet::Moon => Some((w / 9.81) * 1.622),
    }
}
