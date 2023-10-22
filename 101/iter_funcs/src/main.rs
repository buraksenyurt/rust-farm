fn main() {
    /*
       iter modülündeki bazı kullanışlı fonksiyonların üstünden geçiyoruz.
    */

    let car_list = vec![
        "Ferari 5-5-5",
        "Lamborcini Gayyardo",
        "Ford Mastenk",
        "Subaru Impreza",
        "Audi 100 Quattro",
        "Mitsubişi Lensir",
        "Porsh 911",
    ];

    let artist_list = vec![
        "Blue Floyd",
        "Ametalika",
        "EmEfÖö",
        "Sanco Ramirez",
        "Minideth",
    ];

    // iter fonksiyonu car_list için bir iterator döndürür
    let mut car_iter = car_list.iter();
    // // #01 en basit haliyle bir for döngüsünde tüm listeyi gezebiliriz
    // for car in car_iter {
    //     println!("{car}");
    // }

    // #02 iterator üstünden sıradaki elemanı almak için next fonksiyonunu kullanabiliriz.
    // son elemanı almak içinse last fonksiyonundan yararlanabiliriz.
    car_iter.next(); // ilk elemana geçer
    let car = car_iter.next(); // ilk elemandan sonrakine geçer
    println!("\n#02 - next ve last kullanımları.");
    println!("Listedeki ikinci araba '{}'.", car.unwrap());
    println!("Listedeki son araba '{}'.", car_iter.last().unwrap());

    // #03 İki iterator'u birleştirmek için chain fonksiyonunu kullanabiliriz.
    // bu yeni bir iterator oluşturur. Örnekte müzisyen ve araba listeleri birleştirilir
    println!("\n#03 - chain ile arka arkaya koyalım.");
    let entities = artist_list.iter().chain(&car_list);
    for entity in entities {
        println!("{}", entity);
    }

    // #04 bir iterasyona tabi olan elemanları başka bir tipe dönüştürürken
    // map fonksiyonundan yararlanılabilir.
    // aşağıdaki örnekte önce &&str türünden String türüne dönüşüm yapılmış
    // sonrasında elde edilen yeni iterasyondaki her bir elemanın sonuna
    // ' is ready to go!' metni eklenmiştir.
    // Son olarak da yeni iterasyondaki her bir elemanı ekrana yazdıracak şekilde
    // for_each fonksiyonuna başvurulmuştur.
    println!("\n#04 - transform ve for_each kullanımı.");
    car_list
        .iter()
        .map(|c| String::from(*c))
        .map(|mut c| {
            c.push_str(" is ready to go!");
            return c;
        })
        .for_each(|c| println!("{}", c));

    // #05 Bazı durumlarda bir iterasyona tabi olan elemanları adım sayısı ile dolaşmak
    // isteyebiliriz. Örneğin 2şer 2şer veya 3er 3er. Yani iki elemanda bir ilerleyen bir
    // iterasyon kullanmak istersek step_by fonksiyonuna başvurabiliriz.
    println!("\n#05 - 2 şer 2 şer atlayarak araç listesini dolaşalım.");
    let mut with_step = car_list.iter().step_by(2);
    println!("{}", with_step.next().unwrap());
    println!("{}", with_step.next().unwrap());
    println!("{}", with_step.next().unwrap());

    // #06 Eş elemana sahip iki listenin elemanlarını eşleştirip tuple türünde çiftlerden oluşan
    // yeni bir liste elde etmek için zip fonksiyonundan yararlanabiliriz.
    // Aşağıdaki örnekte game_list ve game_orders elemanları birbirleri ile eşleştirilerek
    // yeni bir iterasyon elde edilmektedir.
    let game_list = vec![
        "Prince of Persia",
        "Super Mario",
        "Emlyn Hughes Soccer",
        "Barbarian",
        "Donkey Kong",
        "Pac-Man",
        "Space Invaders",
        "Maniac Mansion",
        "The Legend of Zelda",
        "Galaga",
        "Pitfall",
        "Boulder Dash",
        "Paper Boy",
        "Green Barets",
    ];
    println!("\n#06 - zip ile birleştirelim.");
    let game_orders = vec![1, 5, 6, 4, 2, 3, 7, 10, 9, 11, 8, 14, 12, 13];
    let games = game_list
        .iter()
        .map(|g| String::from(*g))
        .zip(game_orders.iter().map(|o| *o));
    games
        .clone()
        .for_each(|g| println!("{} - {}. sırada oynanacak.", g.0, g.1));

    // #07 Bir iterasyondaki elemanları indek numaraları ile dolaşmak için enumerate
    // fonksiyonundan aşağıdaki gibi yararlanabiliriz.
    println!("\n#07 - enumerate ile indeksleyelim.");
    for (index, pair) in games.clone().enumerate() {
        println!("{} -> {} - {}", index, pair.0, pair.1);
    }

    // #08 iterasyonda belli sayıda elemanı atlayıp ilerlemek için skip kullanılabilir
    // genelde sayfalama(paging) gibi tekniklerde take fonksiyonu ile birlikte kullanılır.
    println!("\n#08 - skip ile atlayıp take ile belli sayıda eleman alalım.");
    games
        .skip(5)
        .take(5)
        .for_each(|g| println!("{} = {}", g.0, g.1));

    // #09 Bir listedeki elemanların belli öğeleri ile bir hesaplama işlemi yapmak istersek
    // fold fonksiyonundan yararlanabiliriz. Aşağıdaki örnekte chart_list içindeki ürünlerin
    // toplam stok değerlerinin toplamı bulunmaktadır
    let chart_list = vec![
        ("Pencil 0.5", 25),
        ("A4 Paper Set", 10),
        ("Wireless ElCi Mouse", 16),
        ("AyFon 29SE", 3),
    ];
    // fold iki parametre alır. İlki hesaplamada kullanılacak değerin ilk halidir.
    // İkinci parametrede iki parametre ile çalışan bir closure söz konusudur. Bu fonksiyonun
    // ilki hesaplamada kullanılacak değişkeni ikincisi de iterasyondan gelen öğeyi temsil eder.
    let stock_count = chart_list
        .clone()
        .iter()
        .fold(0u32, |quantity, item| quantity + item.1);
    println!("\n#09 - fold ile stoktaki toplam ürün sayısını bulalım.");
    println!("Stokta toplam {stock_count} ürün var.");

    // #10 Iterasyon sırasında sonradan gelen bir elemean var mı varsa ele alalım gibi
    // bir ihtiyacımız olabilir. Bu durumda peekable fonksiyonu ile yeni bir iterasyon hazırlayıp
    // peek fonksiyonunun sonucunda dönen Option değeri kontrol edilebilir.
    // Aşağıdaki örnekte ürün listesinin son elemanına kadar ilerlenmektedir. Son elemandan sonra
    // peek ile bir eleman olup olmadığına bakılabilir.
    // peek ile next arasında önemli bir fark vardır. next ile sonraki elemana gidilirken iterasyon
    // da ilerler. peek ise sadece sonraki elemana bakılmasını sağlar ama iterasyonu ilerletmez.
    let mut product_iter = chart_list.iter().peekable();
    product_iter.next();
    product_iter.next();
    product_iter.next();
    product_iter.next();
    if let Some(item) = product_iter.peek() {
        println!("{}", item.0);
    } else {
        println!("Ürün listesinin sonunda olmalıyız. Devamında bir eleman bulunamadı");
    }
}
