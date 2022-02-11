use std::io;

fn main() {
    /*
    Ekran bir girdi alıp bunu iki farklı fonksiyona yolladığımız bir kod parçası var.
    Fonksiyonlardan birisi referans ile çalışıyor yani değişkeni ödünç alıp kullanıyor(Borrowing)
    Diğer fonksiyon ise parametre olarak gelen neseyi sahiplenerek kullanıyor(Ownership)

    GDB ile scope'lardaki durumu inceliyoruz.
     */
    println!("En sevdiğin çizgi karakter kimdir?");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Ekrandan okumada hata!");

    /*
       Fonksiyon sırasına dikkat. Aşağıdaki sırada ilk fonksiyonda ownership sonucu
       user_input ortada kalmaz.
    */
    // whats_say_you(user_input);
    // say_what(&user_input);

    /*
       Aşağıdaki kullanımda ise sıkıntı olmaz çünkü say_what'a user_input geçici olarak
       emanet edilir. Sahipliği sonrasında diğer metoda verilmektedir.
    */
    say_what(&user_input);
    whats_say_you(user_input);
}

// Ownership kullanan fonksiyon
fn whats_say_you(input: String) {
    println!("{}", input.to_uppercase());
}

// Borrowing kullanan fonksiyon
fn say_what(input: &String) {
    println!("{}", input.to_uppercase());
}
