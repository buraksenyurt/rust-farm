/*
    Senaryo gereği üç ırk var. Klingon'lar, Gungan'lar ve Elf'ler.
    Herbiri için uyarlanmasını istediğimiz temel fonksiyonlar olduğunu düşünelim.
    Bu fonksiyonları Entity isimli bir trait içinde tanımladık.
    İlgili struct'lar bu fonksiyonları kendilerine göre implemete etmekte,
    bir başka deyişle override etmekteler.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_test() {
        let kevin = Minion::new(String::from("Pixar"));
        let kahles = Klingon::new(String::from("Star Trek"));
        let arven = Elf::new(String::from("Middle Earth"));

        // say_something fonksiyonu Entity sözleşmesini uyarlayan tiplerle çalışabilir.
        assert_eq!(
            say_something(kevin),
            "Bello globo. Sama to be yambrr com me?"
        );
        assert_eq!(say_something(kahles), "qo' vIvan. tlhIngan jIH tlhInganpu");
        assert_eq!(say_something(arven),"I ambar na- changed; im tur- feel ha in i nen, im tur- feel ha in i coe, im tur- smell ha in i gwilith.");
    }

    // Örneğin aşağıdaki kod parçası derlenmeyecektir.
    // Nitekim Wookie isimli struct Entity isimli trait'i implemente etmemiştir.
    // say_something fonksiyonunun generic tipinin ise Entity trait'ini uygulamış olması gerekir.
    /*    #[test]
    fn trait_not_implement_raise_error_test() {
        let chuwi = Wookie {
            universe: String::from("Star Wars"),
        };
        say_something(chuwi); // the trait `Entity` is not implemented for `Wookie`
    }*/
}

// İşin güzelleştiği yer aşağıdaki fonksiyondur.
// fonksiyon generic bir parametre alır ve onun da Entity türünü uyarlamış olması beklenir.
pub fn say_something<T: Entity>(kind: T) -> String {
    kind.say_hello()
}

// Entity implemente eden türlerin yazmasını istediğimiz fonksiyonları belirttik
pub trait Entity {
    fn new(universe: String) -> Self;
    fn say_hello(&self) -> String {
        String::from("Hello world.")
    }
}

// Klingon veri türü için Entity fonksiyonlarını uyarladık
impl Entity for Klingon {
    fn new(universe: String) -> Self {
        Klingon { universe }
    }

    fn say_hello(&self) -> String {
        String::from("qo' vIvan. tlhIngan jIH tlhInganpu")
    }
}

// Burada da Minion veri türü için uyarladık
impl Entity for Minion {
    fn new(universe: String) -> Self {
        Minion { universe }
    }

    fn say_hello(&self) -> String {
        String::from("Bello globo. Sama to be yambrr com me?")
    }
}

// Son olarak Elf veri türü için uyarladık
impl Entity for Elf {
    fn new(universe: String) -> Self {
        Elf { universe }
    }

    fn say_hello(&self) -> String {
        String::from("I ambar na- ch-+anged; im tur- feel ha in i nen, im tur- feel ha in i coe, im tur- smell ha in i gwilith.")
    }
}

// allow(dead_code) ile universe alanının hiç kullanmadığımız için ilgili uyarıyı yok saydırdık
#[allow(dead_code)]
struct Klingon {
    universe: String,
}

#[allow(dead_code)]
struct Minion {
    universe: String,
}

#[allow(dead_code)]
struct Elf {
    universe: String,
}

#[allow(dead_code)]
struct Wookie {
    universe: String,
}
