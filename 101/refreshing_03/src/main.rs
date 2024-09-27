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
