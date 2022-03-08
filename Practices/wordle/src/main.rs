use colored::Colorize;

/*
   Kelime listesi sadece okuma amaçlı kullanılacak.
   include_str! makrosu parametre olarak gelen dosyayı derleme zamanında alıp kaynak kodun içerisine gömer.
   Dolayısıyla data dosyasını release aldıktan sonra programı götürdüğümüz yere taşımaya gerek yoktur.
*/
const WORDS: &str = include_str!("words.data");

fn main() {
    let wellcome = "World oyununun klonuna hoş geldiniz.".yellow();
    println!("{}", wellcome);
}
