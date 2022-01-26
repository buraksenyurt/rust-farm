fn main() {
    let mut score = 23;
    let other = &mut score; // burada score verisinin sahipliği geçici olarak other'a aldık.

    // Şimdi sahiplik geçici olarak other değişkeninde
    // * ile değere erişip değiştirebiliriz.
    *other += 1;
    println!("Other :{}", *other);
    // ve bu değişiklik doğal olarak sahipliğini geçici olarak aldığımız orjinal değişkenin
    // verisini de değiştirmek anlamına gelir
    println!("Score :{}", score); // her ikisi de 24 oldu

    // Dikkat edilmesi gereken bir durum
    let mut start = 1;
    let _process = &start; // sahipliği geçici olarak process'e verdik
                           // Alt satırda bir atama deniyoruz
    start += 1;
    println!("Start {}", start);
    // atama mümkün görünüyor ama bir alt satırda process'in işaret ettiği veriye erişim yapılmak isteniyor.
    // Sahiplik process'te iken ve onun verisini kullanmaya çalışırken
    // sahipliğini aldığımız değişkenin değerini değiştirmek istediğimizi rust derleyicisi fark ediyor.
    // cannot assign to `start` because it is borrowed
    // println!("Process :{}", *_process);

    // Birde ownership örneğindeki gibi vector kullanımındaki duruma bakalım
    // Gayet masumane bir şekilde vector içeriğini dolaşırken print! ile değerlerini ekrana basıyoruz.
    let numbers = vec![32, 23, 11, 1, 8];
    for n in numbers {
        print!("{}\t", n);
    }
    // Sonrasında da numbers'ın içeriğin yine ekrana bastırmak istiyoruz.
    println!("{:?}", numbers);
    // Yukarıdaki kullanımda `numbers` moved due to this implicit call to `.into_iter()`
}
