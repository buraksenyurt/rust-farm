fn main() {
    // score değişkeninin tuttuğu değer other'a kopyalanır.
    // Primitive tipler için bu varsayılan atama davranışıdır.
    let mut score = 23;
    let other = score;
    println!("Score :{}, Other:{}", score, other);
    // Dolayısıyla örneğin score değişkeninin tuttuğu değer değiştirildiğinde
    // atama yapılan diğer değişken bundan etkilenmez
    score = 24;
    println!("Score :{}, Other:{}", score, other);

    // Birde aşağıdaki vector kullanımındaki sahiplik durumuna bakalım
    let numbers = vec![10, 23, 24, 32, 11];
    // Aşağıdaki atamam sonrası numbers değişkeninin işaret ettiği veri alanının sahipliği,
    // others isimli değişkene geçer.
    let others = numbers;
    println!("{:?}", others);
    // ownership kuralı gereği bir veri parçasının tek bir sahibi olabilir.
    // vector yapısı Copy trait'inin de uyarlamadığından bir veri kopyalaması söz konusu değildir.
    // yukarıdaki atama sebebiyle aşağıdaki satır için derleme hatası alınır.
    // çünkü numbers'ın işaret ettiği veri parçasının sahipliği others'a geçmiştir.
    // println!("{:?}", numbers); //value borrowed here after move

    // Şimdi aşağıdaki kullanımı ele alalım.
    // points isimli bir vector hm_pass isimli fonksiyona gönderiliyor
    let points = vec![25, 50, 70, 100, 10, 0];
    let hm_pass = |data: Vec<i32>| -> Vec<i32> {
        // fonksiyon içerisindeyken points'in sahipliği data isimli değişkene geçecektir
        println!("Öğrenci puanlarına göre atamalar yapılıyor");
        data // ve data isimli değişken burada geriye döndürülmektedir.
    }; // burada da closure scope'u sonlanmaktadır ve kurallara göre data değişkeni ölür.
       // sahiplik yeniden points'e verilir.
       // Bu yüzden aşağıdaki kullanımda herhangi bir kural ihlali olmaz.
    let points_again = hm_pass(points); // points, hm_pass içerisine gönderiliyor.
    println!("{:?}", points_again);

    // Yukarıda closure üzerinden baktığımız senaryo main dışı fonksiyon çağrımında da geçerlidir.
    let first_five = vec![32, 11, 1, 23, 24];
    // get_stats metodu aşağıdaki kullanım sırasında first_five vecktörünün işaret ettiği veri
    // parçasının sahipliğini alır.
    let stats = get_stats(first_five);
    println!("{:?}", stats);
}

fn get_stats(data: Vec<i32>) -> Vec<i32> {
    println!("İlk beş istatistikleri toplanıyor");
    data // fonksiyon gelen vecktörü geriye döndürmektedir
} // data bu noktada ölürken verinin sahipliği tekrar main fonksiyona(callar) döner
