use std::rc::Rc;

fn main() {
    let full_name = Rc::new(String::from("Süıpırmen"));
    println!(
        "full_name nesnesi oluşturuldu. Rreferans sayısı {}",
        Rc::strong_count(&full_name)
    );

    // Deneysel olarak bir scope açıldı
    {
        // full_name değişkeni Rc türünden tanımlandı.
        // ownership taşımak yerine referansı çoğalttık.
        let supo = Hero::new(full_name.clone(), 88);
        println!(
            "Scope içindeyiz ve Hero nesnesi oluşturuldu. full_name için referans sayısı {}",
            Rc::strong_count(&full_name)
        );
        println!("{}", supo.say_hello());

        let justice_league = Journey::new(supo, String::from("Justice Leauge"));
        println!("{}", justice_league.start());
        println!(
            "Scope içindeyiz ve Journey nesnesi oluşturuldu. full_name için referans sayısı {}",
            Rc::strong_count(&full_name)
        );
    } // Scope dışına çıktığımız yer. full_name için referans sayısı bir eksilecek

    println!(
        "Scope dışına çıktık ve full_name için referans sayısı {}",
        Rc::strong_count(&full_name)
    );
}

// Hero yapısındaki full_name değişkeni, Reference Counted türden tanımlanmıştır.
// Rc aslında single-thread reference couting pointer olarak ifade edilir.
struct Hero {
    full_name: Rc<String>,
    level: u8,
}

struct Journey {
    hero: Hero,
    title: String,
}

impl Hero {
    fn new(full_name: Rc<String>, level: u8) -> Self {
        Hero { full_name, level }
    }
    fn say_hello(&self) -> String {
        format!(
            "Merhaba. Ben {}. seviyeden {}",
            &self.level, &self.full_name
        )
    }
}

impl Journey {
    fn new(hero: Hero, title: String) -> Self {
        Journey { hero, title }
    }
    fn start(&self) -> String {
        format!(
            "{}. {} macerası başlıyor",
            &self.hero.say_hello(),
            &self.title
        )
    }
}
