#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_new_change_request_info_is_empty() {
        let mut rfc_1 = ChangeRequest::new();
        rfc_1.add_info(
            "Menü arka plan rengi".to_string(),
            "Menü arka planının kullanıcı isteğine göre değişebilmesini istiyorum".to_string(),
        );
        assert_eq!(rfc_1.get_info(), "");
    }

    #[test]
    fn should_next_state_after_draft_is_waiting_for_approval() {
        let mut rfc_1 = ChangeRequest::new();
        rfc_1.add_info(
            "Menü arka plan rengi".to_string(),
            "Menü arka planının kullanıcı isteğine göre değişebilmesini istiyorum".to_string(),
        );
        let current_info = rfc_1.get_info();
        assert_eq!(current_info, "".to_string());
        rfc_1.request_approve();
        let current_info = rfc_1.get_info();
        assert_eq!(current_info, "Onay Bekliyor|Menü arka plan rengi|Menü arka planının kullanıcı isteğine göre değişebilmesini istiyorum".to_string());
    }
}

// Durumlar arasındaki geçişlere ait davranışları tanımlayacağımız trait
trait State {
    // ChangeRequest nesne içeriğini çekmek için bildirilen davranış.
    // Varsayılan olarak boş bir string referansı döner.
    // Bu davranışı State trait'ini implemente eden WaitingForApproval ile Committed
    // veri yapıları yeniden uygulayabilirler. Böylece nesnenin bulunduğu duruma ait değerleri
    // elde edebiliriz.
    // information fonksiyonu bir gövdeye sahip olduğu için State'i implemente eden her struct
    // için yeniden yazılmak zorunda değildir. Mesela Draft veri yapısı için uyarlanmamıştır,
    // nitekim zaten Draft durumundayken title ve description bilgilerinin boş olması istenmektedir.
    fn information(&self, _cr: &ChangeRequest) -> String {
        "".to_string()
    }
    // Draft halinden ChangeForApproval'a geçişi tanımladığımız davranış
    fn request_approve(self: Box<Self>) -> Box<dyn State>;
}

// ChangeRequest nesnesinin ilk halini ifade eden veri yapısı
#[derive(Debug)]
struct Draft {}
// Sonraki duruma geçiş de dahil olmak üzere State üstünden gelen davranışlar uyarlanıyor
impl State for Draft {
    fn request_approve(self: Box<Self>) -> Box<dyn State> {
        // Draft için bu davranışın uygulanması bir sonraki durum olan WaitingForApproval'a geçiştir.
        Box::new(WaitingForApproval {})
    }
}

// Draft modundan sonra geçilebilecek olan durumu ifade eden veri yapısı
#[derive(Debug)]
struct WaitingForApproval {}
// Bu veri yapısı içinde State trait'inde tanıumlı davranışları uyarlamalıyız.
impl State for WaitingForApproval {
    // ChangeRequest nesnesi bu konumdayken zaten request_approve gibi bir davranışı uygulamamalıdır.
    fn request_approve(self: Box<Self>) -> Box<dyn State> {
        // bu nedenle kendisini döndürür
        self
    }
    // Nesnemiz WaitingForApproval durumuna geçtiğinde artık title ve description gibi bilgilerin
    // görülebilmesi gerekir. Bu nedenle State trait'i içindeki information davranışını yeniden
    // yazmaktayız.
    fn information(&self, cr: &ChangeRequest) -> String {
        format!("Onay Bekliyor|{}|{}", cr.title, cr.description)
    }
}

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

    // Nesnenin bilgilerini çekmek için çağırılan fonksiyon.
    // Dikkat edilmesi gereken nokta nesnenin o an sahip olduğu durum bilgisine göre
    // information fonksiyonunun çağırılmakta oluşudur.
    // Örneğin nesne o anda WaitingForApproval veya Committed durumunda ise bu veri yapılarında
    // State trait'inden gelip yeniden yazılan information fonksiyonu çağırılır.
    pub fn get_info(&self) -> String {
        self.state.as_ref().unwrap().information(self)
    }

    // Nesnemizin Draft durumundan WaitingForApproval durumuna geçişi sırasında kullanılacak fonksiyon
    pub fn request_approve(&mut self) {
        // Eğer nesne durumunu alabiliyorsak
        if let Some(s) = self.state.take() {
            // O anki trait kimse onun üstünden asıl durum nesnesinin uyguladığı request_approve fonksiyonu çağırılsın.
            self.state = Some(s.request_approve())
        }
    }
}
