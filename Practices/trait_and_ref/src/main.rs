fn main() {

    // Bölüm 1: Basit bir trait tanımlanır ve bir veri yapısı için uygulanıp kullanılır
    let rocky = Climber;
    rocky.teach();
}


trait Trainer{
    fn teach(&self); 
}

struct Climber;

impl Trainer for Climber{
    fn teach(&self) {
        println!("Sana, nasıl dağa tırmanılır onu öğreteceğim.")
    }
}
