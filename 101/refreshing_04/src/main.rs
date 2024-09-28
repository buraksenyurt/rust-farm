/*
   Bu örnekte lifetime konusunu kısaca hatırlamaya çalışıyoruz.

   Oluşturduğumuz her değişkenin(variable) bir yaşam süresi olur (lifetime)
   Lifetime, bir değişkenin ne kadar süre kullanılabileceğini belirtir.
   Bilindiği üzere değişkenler skop dışında kaldıklarında otomatik olarak bellekten düşürülürler.

   main fonksiyonu içimnde colors isimli bir vektör oluşturuluyor. Bellekte buna göre yer ayrılıyor.
   next metod çağrısı yapıldığında colors değişkeninin referansı ilgili metoda paslanıyor.
   Metot içinde bu referans vektör elemanları dolaşılıyor. Bulunan eleman &str olarak dönüyor.
   &str tahmin edileceği üzere colors vektöründeki bir öğeyi referans etmekte.

   Peki ya colors next çağrısından sonra bir şekilde scope dışına çıkarsa? Bu durumda,
   next_color değişkeni, bellekten drop edilmiş bir vector elemanını referans etmeye devam edecektir.

   Örnekte 30ncu satırda açılan blok bir scope oluşturur. colors vektörü burada oluşturulur ve
   pas edildiği next metodu burada çağrılır. Elde edilen sonuç, next_color değişkenine atanır ve
   bu değişkene scope dışından erişilir. next_color scope dışında tanımlandığı için
   kullanılabilir ancak colors drop edileceğinden artık olmayan bir referansı işaret eder. Yani,
   colors isimli vektörün yaşam süresi, next_color'ın yaşam süresinden daha kısadır. Bu durum
   rust'ın lifetime kuralları gereği bir ihlalldir ve en temelde "Use After Free" hatasının
   da kaynağıdır. O yüzden derleme esnasında hata alınır.

*/

fn main() {
    let next_color;
    {
        let colors = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
            String::from("yellow"),
            String::from("magenta"),
            String::from("cyan"),
            String::from("white"),
            String::from("black"),
        ];
        /*
           next fonksiyon çıktısı kasıtlı olarak String türüne çevrilmiştir.
           Eğer &str olarak bırakırsak, lifetime annotation eklemeleri yapsak bile,
           scope kullanımı nedeniyle next_color, println! fonksiyonunda kullanılamayacaktır, zira
           bellekten düşürülmüş olacaktır.
        */
        next_color = next(&colors, "magenta").to_string();
    } // colors değişkeni drop edilir.
    println!("{}", next_color);

    let games = vec![
        Game {
            name: "Age of Empires III",
            popularity: 980,
        },
        Game {
            name: "Warcraft II",
            popularity: 1750,
        },
        Game {
            name: "Dredge",
            popularity: 1250,
        },
        Game {
            name: "Sensible Soccer",
            popularity: 785,
        },
    ];
    if let Some(game) = get_most_popular(&games) {
        println!(
            "{} is most popular game. Game point is {}",
            game.name, game.popularity
        );
    }
}

/*
   Aşağıdaki örnekte parametre olarak gelen String türevli vector slice içinden
   bir sonraki elemanı elde ettiğimiz örnek bir işlev yerine getirilmektedir.
   Metodumuzun parametreleri ve dönüşü referans türleridir.
*/
fn next<'a>(colors: &'a [String], current: &str) -> &'a str {
    let mut found = false; // Elemanın bulunup bulunmadığı bilgisi
    for color in colors {
        // Bu metoda referans olarak verilmiş colors vektörü dolaşılıyor
        if found {
            // flag true ise o anki eleman döndürülüypr
            return color;
        }
        if color == current {
            // color == curent ise bir sonraki elemanı döndürmek üzere flag true yapılıyor
            found = true;
        }
    }

    colors.last().unwrap()
}

// // Version 00 (Lifetime hatası olan)
// fn next(colors: &[String], current: &str) -> &str {
//     let mut found = false; // Elemanın bulunup bulunmadığı bilgisi
//     for color in colors {
//         // Bu metoda referans olarak verilmiş colors vektörü dolaşılıyor
//         if found {
//             // flag true ise o anki eleman döndürülüypr
//             return color;
//         }
//         if color == current {
//             // color == curent ise bir sonraki elemanı döndürmek üzere flag true yapılıyor
//             found = true;
//         }
//     }
//
//     colors.last().unwrap()
// }

/*
   Kendi veri modellerimizde metinsel türlerde String yerine &str gibi referans türleri
   kullanmaya başladığımız andan itibaren lifetime annotation'lar gerekli olur.

   Bu durumda Game nesne referansı ile çalışan yerlerde de lifetime annotation'lar gerekecektir.
   Söz gelimi get_most_popular fonksiyonunda.
*/
#[derive(Debug)]
struct Game<'a> {
    name: &'a str,
    popularity: i32,
}

fn get_most_popular<'a>(games: &'a [Game]) -> Option<&'a Game<'a>> {
    games.iter().max_by_key(|g| g.popularity)
}

/*
   İlginç bilgi: lifetime konusunda remove kelimesi yerine elision kullanılıyormuş.
   Mesela, "You can remove the annotations" yerine "You can elide the annotations" gibi.
   Hatta aşağıdaki fonksiyonu göz önüne alalım. Burada parametre ve dönüş &str türünden.
   Bu gibi durumlarda açıkça lifetime annotation belirtilmesine gerek olmaz.
   Bunun için rust şöyle bir öneride bulunur;

   "Parameter types contain explicit lifetimes that could be elided"
*/
// fn find_first_word<'a>(value: &'a str) -> &'a str {
//     value.split_whitespace().next().unwrap()
// }

fn find_first_word(value: &str) -> &str {
    value.split_whitespace().next().unwrap()
}
