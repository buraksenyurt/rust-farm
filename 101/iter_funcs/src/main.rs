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

    // #02 iterator üstünden sıradaki elemanı almak için next fonksiyonunu kullanabiliriz
    car_iter.next(); // ilk elemana geçer
    let car = car_iter.next(); // ilk elemandan sonrakine geçer
    println!("Listedeki ikinci araba '{}'", car.unwrap());

    // #03 İki iterator'u birleştirmek için chain fonksiyonunu kullanabiliriz.
    // bu yeni bir iterator oluşturur. Örnekte müzisyen ve araba listeleri birleştirilir
    let aggregation = artist_list.iter().chain(&car_list);
    for agg in aggregation {
        println!("{}", agg);
    }
}
