#[cfg(test)]
mod tests {
    use crate::ChangeRequest;

    #[test]
    fn should_new_change_request_info_is_empty() {
        let mut rfc_1 = ChangeRequest::new();
        rfc_1.add_info(
            "Menü arka plan rengi".to_string(),
            "Menü arka planının kullanıcı isteğine göre değişebilmesini istiyorum.".to_string(),
        );
        assert_eq!(rfc_1.get_info(), "");
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
    // Taleple ilgili bilgileri eklemek için bir fonksiyondan yararlanacağız.
    pub fn add_info(&mut self, title: String, description: String) {
        self.title = title;
        self.description = description;
    }
    // ChangeRequest, Draf moddayken hiçbir içeriğie sahip olmamalı şeklinde bir kuralımız vardı.
    // Dolayısıyla bu bilgiler, ChangeRequest nesnesi ancak bir sonraki state'e geçerse kullanılabilir olmalı.
    // Bu nedenle get_info fonksiyonu boş bir string referans döner.
    pub fn get_info(&self) -> &str {
        ""
    }
}
