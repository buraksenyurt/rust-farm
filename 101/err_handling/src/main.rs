fn main() {
    /*
       Bazen beklemediğimiz ya da kontrol altına alamadığımız bir hata oluşabilir. Bu durum,
       rust çalışma zamanı için panik anlamına gelir. Örneğin sınır dışı bir indis değerini kullanmak,
       olmayan dosyayı açmaya çalışmak gibi.

       İlk kod parçasında vecktor nesnesinin olmayan 10ncu indisine ulaşışmaya çalışılıyor.

    */
    let numbers = vec![3, 5, 1, 6, 9, 11];
    let number = numbers[10];
    println!("{}", number);
}
