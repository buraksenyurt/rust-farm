/*
    Iterator kullanımını hatırlayalım.
*/

fn main() {
    let mut levels = vec![
        String::from("beginner"),
        String::from("intermediate"),
        String::from("upperIntermediate"),
        String::from("professional"),
        String::from("master"),
    ];

    // let mut levels_iterator = levels.iter();

    // println!("{:?}", levels_iterator.next());
    // println!("{:?}", levels_iterator.next());
    // println!("{:?}", levels_iterator.next());
    //
    // for level in levels_iterator {
    //     println!("Level: {}", level);
    // }
    write_all(&levels);

    println!();

    write_all_nicely(&levels[2..=4]); //2nciden itibaren 4ncü dahil liste elemanlarını alan slice

    convert_shorten(&mut levels);

    write_all(&levels);

    println!();

    let new_list = convert_to_uppercase(&levels);
    write_all(&new_list);

    /*
       into_iter kullandığımızda sahiplik(ownership) de taşınır.
       Aşağıdaki senaryoda locations değişkeni ile ifade edilen vektör elemanları
       into_iter adaptörü ardından gelen filter tüketicisi ile B harfine göre çekilirler
       ve her biri for_each tüketicisi ile target isimli vektöre taşınır(move)

       Bu nedenle de 55nci satırdaki kod çalışmaz.
       Nitekim locations elemanları target'a taşınmıştır ve locations
       değişkeni artık kullanılabilir değildir.

    */
    let locations = vec![
        String::from("Tokyo"),
        String::from("İstanbul"),
        String::from("London"),
        String::from("Berlin"),
        String::from("Beijing"),
        String::from("Budapest"),
        String::from("Boston"),
    ];
    let mut target = vec![];
    locations
        .into_iter()
        .filter(|l| l.starts_with("B"))
        .for_each(|arg| target.push(arg));
    println!("Locations {:?}", target);
    // println!("Locations: {:?}", locations); // Value used after being moved

    /*
       Aşağıdaki örnekte String türüdünden elemanlar taşıyan bir vector'ün
       tüm elemanları char türünden birer vector türüne dönüştürülmektedir.
    */
    let team_codes = vec![
        String::from("REDONE"),
        String::from("BLUETWO"),
        String::from("COPYTHAT"),
        String::from("IRINE"),
        String::from("BLACKDOWN"),
        String::from("GOLDLEADER"),
        String::from("RANGERECHO"),
        String::from("SIERRAALPHA"),
    ];
    /*
       collect fonksiyonu ne döneceğini bilmek ister. Bunun için type annotation kullanır.
       Aşağıdaki kullanımda bunu yapmazsak "type must be known at this point" hatası alırız.
    */

    let coded: Vec<Vec<char>> = team_codes
        .iter()
        .map(|a| a.chars().map(|c| c.to_ascii_lowercase()).collect())
        .collect();

    coded.iter().for_each(|l| println!("{:?}", l));

    let founded_team_code = search_team_codes(&team_codes, "BLUE", "OFF");
    println!("Found team code {:?}", founded_team_code);
    let founded_team_code = search_team_codes(&team_codes, "PINK", "OFF");
    println!("Found team code {:?}", founded_team_code);

    let gamers = vec![
        Player {
            name: String::from("Rouge one"),
            points: 10,
        },
        Player {
            name: String::from("Mor Galled"),
            points: 4,
        },
        Player {
            name: String::from("Condor"),
            points: 5,
        },
        Player {
            name: String::from("Wicker"),
            points: 9,
        },
    ];

    // Collect için bir başka type annotation örneği
    let beginners = gamers
        .into_iter()
        .filter(|g| g.points <= 5)
        .map(|g| g.name)
        .collect::<Vec<String>>();

    beginners.iter().for_each(|b| println!("{}", b));
}

/*
   String türünden bir vector slice üzerinde argument ile belirtilen değer aranır.
   Bulamazsa fallback değeri ele alınır.
   argument ve fallback herhangi bir değişime uğramayacak veya herhangi bir hesaplamada
   kullanılmayacak türden değişkenleri ifade edebilir.
   Bu nedenle string slice olarak tanımlanmışlardır.

   Fonksiyonun nasıl kullanılmak istendiğine göre dönüş türü değişiklik gösterebilir.
   Yani, mutable veya immutable olması tamamen fonksiyonun kullanım amacına bağlı olabilir.
*/
fn search_team_codes(codes: &[String], argument: &str, fallback: &str) -> String {
    codes
        .iter()
        .find(|c| c.contains(argument))
        .map_or(String::from(fallback), |arg| arg.to_string())
    // map_or fonksiyonu bir önceki fonksiyon çağrısının Option değerine göre hareket eder.
    // Eğer sonuç None ise ilk argümanı döner, eğer Some ise(yani bir değer bulunmuşsa) onu döner.
}

fn write_all(list: &Vec<String>) {
    /*
       Elemenlarında ileri doğru hareket edebileceğimiz herhangibir kaynak için
       basit for döngüsü kullanabiliriz. for döngüsü otomatik olarak
       bir iterator nesnesi örnekler, onun üzerinden next operasyonunu işletir
       ve unwrap ile Option yoluyla gelen değeri alır.
       Ve bunu None ile karşılaşana kadar devam ettirir.
    */
    // for argument in list {
    //     println!("{}", argument);
    // }

    /*
       Bir başka yol da iterator adaptörlerini ve tüketicilerini(consumers) kullanmaktır.
       for_each, map, collect örnek Consumer fonksiyonlarıdır.
       Bu fonksiyonlar Closure olarak adlandırılan ve C#çılar için anonymous method
       olarak da ifade edebileceğimiz enstrümanlardır.

       Iterator' lar aslında lazy yapılardır. Yani arkasından next çağrısını kullanan
       bir başka method(Consumer) gelinceye kadar bir şey yapmaz.
    */
    list.iter().for_each(|arg| println!("{}", arg));

    /*
       Örnek Iterator Adaptor kullanımı.
       map fonksiyonu örnek bir iterator adaptor' dur.
       Adaptor'ler pipeline üzerinde bir işlem tanımlarlar.
       Örneğin her bir argümanı alıp formatlı bir şekilde yazdırmayı bir işlem olarka tanımlarlar.
       (Aşağıdaki format kullanımına bakabiliriz)
       map lazy bir fonkisyondur. Yani arkasından bir Consumer çağrısı gelmediği sürece
       döngüsel iterasyon başlamaz. Consumer fonksiyon(for_each gibi) map ile tanımlanan işlemi
       tüm elemanlar için uygulayarak iterasyonu geçekleştirir.
    */
    // list.iter()
    //     .map(|arg| format!("{} is {} characters", arg, arg.len()))
    //     .for_each(|r| println!("{}", r));
}

// Vector slice kullanımı
fn write_all_nicely(list: &[String]) {
    list.iter().for_each(|arg| println!("{}", arg));
}

/*
   iterator fonksiyonlarında duruma göre farklı versiyonlar kullanılır.
   Eğer kaynak üzerinde değişiklik gerekiyorsa mutable versiyon kullanmak gerekir.

   iter()      :   Elemanlar için read-only referanslar oluşturur
   iter_mut()  :   Elemanlar üzerinde değiştirilebilir(mutable) referans oluşturur
   into_iter() :   Elemanların sahipliği alınır.
*/
fn convert_shorten(list: &mut [String]) {
    list.iter_mut().for_each(|arg| arg.truncate(3))
}

fn convert_to_uppercase(list: &[String]) -> Vec<String> {
    /*
       Collect fonksiyonu da bir iterator consumer'dır.

       Aşağıdaki örnekte map arkasından collect gibi bir Consumer
       çağrılmadığı sürece listenin büyük harfe çevrilmiş hali döndürülemez.

       collect, farklı veri yapılarından değerler toplayabilme özelliği ile de öne çıkar.
       collect fonksiyonu öncesinde çağırılan adaptörden nasıl bir tip üreteceğini anlamak için
       type annotation'lardan yararlanır.

       Bunun için fonksiyonun dönüş türüne bakması yeterlidir.
    */

    list.iter().map(|arg| arg.to_uppercase()).collect()

    // let result = list.iter().map(|arg| arg.to_uppercase()).collect();
    // result
}

struct Player {
    name: String,
    points: i32,
}
