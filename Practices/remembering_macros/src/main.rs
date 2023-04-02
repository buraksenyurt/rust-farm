/*
   makroları main fonksiyonu öncesinde tanımlıyoruz. main altında tanımlarsak;
   error: cannot find macro `ahoy` in this scope

   Matcher'ları regular expression yazmaya benzer ama aynı şey değildir tabi.
   Matcher'ları belli bir kelime veya kelimeler dizesini yakalamak için kullanırız.
*/
macro_rules! ahoy {
    // matcher => transcriber
    () => {};
    // Transcriber tarafına $ ile matcher'ın yakaladığı generic parametreleri taşıyabiliriz.
    // expr bir metavariable'dır. geçerli tüm rust expression'larını temsil eder.
    // Kullanılabilir metavariable listesi için -> https://doc.rust-lang.org/reference/macros-by-example.html#metavariables
    ($name:expr) => {
        println!("ahoy {}", $name)
    };
}

macro_rules! goldsum {
    // $x ve $y toplama işlemi için uygun veri türleri olduğu sürece makro hata vermez.
    ($x:expr,$y:expr) => {
        $x + $y
    };
}

// makro girdisindeki add veya sub kelimelerine göre farklı çalışan bir makro da tasarlayabiliriz.
macro_rules! goldcalc {
    (add $x:expr,$y:expr) => {
        $x + $y
    };
    (sub $x:expr,$y:expr) => {
        $x - $y
    };
}

/*
   vec![] makrosunda olduğu gibi tekrarlı ifadeler kullanmak istediğimiz hallerder
   aşağıdaki notasyonlardan yararlanabiliriz.

   $()* : 0 veya n sayıda girdi alır
   $()+ : en az 1 veya daha çok girdi alır
   $()? : girdi seçime bağlıdır
*/

macro_rules! acc{
    // 1 veya N sayıda expression girilebilir ve bunların toplamı döndürülür.
    // ifadeler arasında virgül olması gerekir
    ($($value:expr),+) =>{
        {
            let mut total=0;
            $(
                total += $value;
            )+
            total
        }
    };
}

macro_rules! gauss {
    ($tail:expr=> $head:expr) => {{
        let mut total = 0;
        for n in $tail..$head {
            total += n;
        }
        total
    }};
}

fn main() {
    ahoy!("bootstrap");
    let treasure = goldsum!(2000, 3500);
    println!("{} gold in our treasure...", treasure);
    // let treasure = goldsum!(12.50, 10); // error[E0277]: cannot add an integer to a float
    let treasure = goldcalc!(add 1000,500);
    println!("{} gold in our treasure...", treasure);
    let treasure = goldcalc!(sub treasure,250);
    println!("{} gold in our treasure...", treasure);
    let treasure = acc!(treasure, 125, 600, 350, 900);
    println!("{} gold in our treasure...", treasure);

    let total = gauss!(1 => 100);
    println!("1 => 100 total {}", total);

    let total = gauss!(10 => 20);
    println!("10 => 20 total {}", total);
}
