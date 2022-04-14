#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Durumlar arasındaki geçişlere ait davranışları tanımlayacağımız trait
trait State {}

// ChangeRequest nesnesinin ilk halini ifade eden veri yapısı
struct Draft {}
// Sonraki duruma geçiş de dahil olmak üzere State üstünden gelen davranışları da implemente edeceğiz.
impl State for Draft {}

// İş akşımızdaki ana nesnemiz. Bir yazılım değişiklik talebini ifade ediyor.
// Basit olması açısında title ve description şeklinde alanlar içermekte.
// Dikkat edileceği üzere state, title ve description alanları private.
// Nitekim nesnenin o anki durumuna ait değerlerin dışarıdan doğrudan müdahele ile değiştirilmemesi gerekiyor.
pub struct ChangeRequest {
    // Nesne durumunu bir Option enum sabiti ile smart pointer kullanarak tanımladık.
    // nesnenin kullanacağı trait çalışma zamanında belli olacağından Dynamic Dispatch yaklaşımı bizim için ideal.
    state: Option<Box<dyn State>>,
    title: String,
    description: String,
}

impl ChangeRequest {
    // ChangeRequest new fonksiyonu ile ilk kez oluşturulurken title ve description boş olmalı.
    // Daha da önemlisi ilk kez oluşturulduğunda Draft durumunda olmalı.
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            title: String::new(),
            description: String::new(),
        }
    }
}
