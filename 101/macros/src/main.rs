use std::f32::consts::PI;
// En basit haliyle bir makro tanımı görüyoruz
macro_rules! bonjorno {
    () => {
        println!("Bonjorno Don Carleone")
    };
}

// Alttaki makro ise bir epxression alır ve bunu ekrana basar.
macro_rules! expresso {
    ($name:expr) => {
        println!("{}", $name)
    };
}

// Alttaki macro n sayıda ifade alıp kullanabilir.
macro_rules! machiato{
    // param isimli ifadeden n tane(*) kadarını ele alır
    ($($param:expr),*)=>
        ($(println!(">\t{}",$param);)*) // ve println! fonksiyonunu parametre sayısı kadar (*) tekrarlar
}

// Bu macro ise x ve y şeklinde iki ayrı eşitliğe atanan ifadeleri yakalar.
macro_rules! latte {
    // x=5,x=4*3 ve benzeri ifadeleri arar. İfadenin x'e eşit olması yeterlidir.
    (x=$e:expr) => {
        // x'e atanan ifadenin sonucunu yazar.
        println!("x = {}", $e)
    };
    // burada da y ismine atanan bir ifade söz konusudur.
    (y=$e:expr) => {
        // bu seferde y'ye atanan ifadenin sonucu yazılır.
        println!("y = {}", $e)
    };
}

// Bu kullanımda ise identifier bilgisinden yola çıkılarak bir fonksiyon inşa edilir.
// Üstelik birde argüman gönderilmekte.
// Dikkatinizi çekerim. Fonksiyon ve argüman derleme sırasında hazırlanıp çalışma zamanına bağlanacak.
macro_rules! barista {
    ($fn_name:ident,$arg:expr) => {
        fn $fn_name() {
            // fonksiyon adı ve birde argümanımız var.
            // Burada çalışması istenen kod bloğu inşa edilebilir.
            println!(
                "{:?} isimli fonksiyon {:?} argümanı ile çalışıyor.",
                stringify!($fn_name),
                $arg
            );
        }
    };
}

fn main() {
    // Örneklerde basit anlamda macro tanımları söz konusudur.
    bonjorno!();

    expresso!("Duble espresso lütfen");
    expresso!(2.22);
    expresso!(true);

    machiato!(-1, 0.0001, "Euroealgue", "Jeniffer", 23, true);

    latte!(x = 3 + 4);
    let r = 10.0;
    latte!(y = PI * r * r);

    barista!(ekrana_yaz, 23);
    ekrana_yaz();
}
