fn main() {
    /*
       Bazen beklemediğimiz ya da kontrol altına alamadığımız bir hata oluşabilir. Bu durum,
       rust çalışma zamanı için panik anlamına gelir. Örneğin sınır dışı bir indis değerini kullanmak,
       olmayan dosyayı açmaya çalışmak gibi.

       İlk kod parçasında vecktor nesnesinin olmayan 10ncu indisine ulaşışmaya çalışılıyor.

    */

    // index out of bounds: the len is 6 but the index is 10
    // let numbers = vec![3, 5, 1, 6, 9, 11];
    // let number = numbers[10];
    // println!("{}", number);

    // Bazen bilinçli olaraktan da çalışma zamanını panic ile kesmek isteyebiliriz.
    // panic!("Canım çalışmak istemiyor");
    // println!("Biraz önce oluşan panic nedeniyle bu satır çalışmayacak");

    let result = divide(6, Some(4));
    println!("6 / 4 = {}", result);

    // Aşağıdaki kullanımda ise panik oluşacaktır.
    let result = divide(6, Some(0));
    println!("6 / 0 = {}", result);

    println!("Yurdaki panic nedeniyle kod buraya gelemeyecek");
}

/*
   Aşağıdaki bölme fonksiyonunu göz önüne alalım.
   Sıfıra bölme işlemi belirsiz kabul edildiğinden bunu istemiyoruz.
   y değerini Option<i32> olarak alıp durumu match pattern uygulayarak kontrol altına alabiliriz.
*/
fn divide(x: i32, y: Option<i32>) -> i32 {
    // 0 olması halinde panic makrosu ile sistemi kesmekteyiz.
    match y {
        Some(0) => panic!("Sıfıra bölme hatası"),
        Some(v) => x / v,
        None => 0,
    }
}
