#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_new_change_request_info_is_empty() {
        // Yeni bir RCF oluşturulduğunda get_info'nun boş bir String dönmesi beklenir.
        // Nitekim nesne Draft durumundadır.
        let mut rfc_1 = ChangeRequest::new();
        rfc_1.add_info(
            "Menü arka plan rengi".to_string(),
            "Menü arka planının kullanıcı isteğine göre değişebilmesini istiyorum".to_string(),
        );
        assert_eq!(rfc_1.get_info(), "");
    }

    #[test]
    fn should_next_state_after_draft_is_waiting_for_approval() {
        // Bu vakada ise yeni bir RFC oluşturulduktan sonra durumu onay bekliyora çekilir.
        // Buna göre title, description gibi bilgiler girildiği gibi alınabilmeli ve hatta
        // nesenin kendisi WaitingForApproval modunda olmalıdır.
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

    #[test]
    fn should_next_state_after_waiting_for_approval_is_committed() {
        // Bu vakada ise yeni bir RFC oluşturulduktan sonra durumu onay bekliyora çekilir.
        // Sonrasında da submit çağrısı ile durumu Committed'a çekilir.
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
        rfc_1.submit();
        let current_info = rfc_1.get_info();
        assert_eq!(current_info, "Onaylandı, Backlog'a eklendi|Menü arka plan rengi|Menü arka planının kullanıcı isteğine göre değişebilmesini istiyorum".to_string());
    }
}

// Durumlar arasındaki geçişlere ait davranışları tanımladığımız trait
trait State {
    // ChangeRequest nesne içeriğini çekmek için bildirilen davranış.
    // Varsayılan olarak boş bir string referansı döner.
    // Bu davranışı State trait'ini implemente eden WaitingForApproval ile Committed
    // veri yapıları yeniden uygulayabilirler.
    // Böylece nesnenin bulunduğu duruma ait değerleri elde edebiliriz.
    // information fonksiyonu bir gövdeye sahip olduğu için State'i implemente eden her struct
    // için yeniden yazılmak zorunda değildir. Mesela Draft veri yapısı için uyarlanmamıştır,
    // nitekim zaten Draft durumundayken title ve description bilgilerinin boş olması istenmektedir.
    fn information(&self, _cr: &ChangeRequest) -> String {
        "".to_string()
    }
    // Draft halinden WaitingForApproval'a geçişi tanımladığımız davranış
    fn request_approve(self: Box<Self>) -> Box<dyn State>;
    // WaitingForApproval durumundan Committed durumuna geçişi tanımlayan davranış
    fn commit(self: Box<Self>) -> Box<dyn State>;
}

// ChangeRequest nesnesinin ilk halini ifade eden veri yapısı
struct Draft {}
// Sonraki duruma geçiş de dahil olmak üzere State üstünden gelen davranışlar uyarlanıyor
impl State for Draft {
    fn request_approve(self: Box<Self>) -> Box<dyn State> {
        // Draft için bu davranışın uygulanması bir sonraki durum olan WaitingForApproval'a geçiştir.
        Box::new(WaitingForApproval {})
    }

    // Nesne, Draft halindeyken doğrudan Committed durumuna geçemez. Bu nedenle self dönülür.
    fn commit(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// Draft modundan sonra geçilebilecek olan durumu ifade eden veri yapısı
struct WaitingForApproval {}
// Bu veri yapısı içinde State trait'inde tanıumlı davranışları uyarlamalıyız.
impl State for WaitingForApproval {
    // ChangeRequest nesnesi bu konumdayken zaten request_approve gibi bir davranışı uygulamamalıdır.
    fn request_approve(self: Box<Self>) -> Box<dyn State> {
        // bu nedenle kendisini döndürür
        self
    }
    // Talep onaylandıktan sonra WaitingForApproval'dan  Committed'a geçebilir
    // Bu nedenle smart pointer üstünden Committed nesne örneği dönülür.
    fn commit(self: Box<Self>) -> Box<dyn State> {
        Box::new(Committed {})
    }
    // Nesnemiz WaitingForApproval durumuna geçtiğinde artık title ve description gibi bilgilerin
    // görülebilmesi gerekir. Bu nedenle State trait'i içindeki information davranışını yeniden
    // yazmaktayız.
    fn information(&self, cr: &ChangeRequest) -> String {
        format!("Onay Bekliyor|{}|{}", cr.title, cr.description)
    }
}

// WaitingForApproval durumundaki sonraki durumu ifade eden veri yapısı
struct Committed {}

// Örnek akışımızda Committed nesnenin içinde olabileceği son haldir.
// Bu nedenle request_approve veya commit gibi davranışlar self ile kendi durumunu döner.
// Nitekim Committed durumunda diğer durumlar geçilmemektedir. En azından bu senaryo gereği.
impl State for Committed {
    fn information(&self, cr: &ChangeRequest) -> String {
        format!(
            "Onaylandı, Backlog'a eklendi|{}|{}",
            cr.title, cr.description
        )
    }

    fn request_approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn commit(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// İş akışındaki ana nesnemiz. Bir yazılım değişiklik talebini ifade ediyor.
// Basit olması açısında title ve description gibi alanlar içermekte.
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
    // ChangeRequest new fonksiyonu ile ilk kez oluşturulurken title ve description alanları boş olmalı.
    // Daha da önemlisi ilk kez oluşturulduğundan nesne Draft durumunda başlamalı.
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            title: String::new(),
            description: String::new(),
        }
    }
    // Taleple ilgili bilgileri eklemek için bir fonksiyondan yararlanabiliriz.
    pub fn add_info(&mut self, title: String, description: String) {
        self.title = title;
        self.description = description;
    }

    // Nesnenin bilgilerini çekmek için çağırılan fonksiyon.
    // Dikkat edilmesi gereken nokta nesnenin o an sahip olduğu durum bilgisine göre
    // information fonksiyonunun çağırılmakta oluşudur.
    // Örneğin nesne o anda WaitingForApproval veya Committed durumunda ise bu veri yapılarında
    // State trait'inden gelip yeniden yazılan information fonksiyonu çağırılır.
    // Dolayısıyla nesnenin içinde bulunduğum durumdaki verisini saklayarak da akışı ilerletme şansımız olur.
    pub fn get_info(&self) -> String {
        self.state.as_ref().unwrap().information(self)
    }

    // Nesnenin Draft durumundan WaitingForApproval durumuna geçişi sırasında kullanılan fonksiyon
    pub fn request_approve(&mut self) {
        // Eğer nesne durumunu alabiliyorsak
        if let Some(s) = self.state.take() {
            // O anki trait kimse onun üstünden asıl durum nesnesinin uyguladığı request_approve fonksiyonu çağırılır.
            self.state = Some(s.request_approve())
        }
    }

    // Nesnenin WaitingForApproval durumundan Committed durumuna geçişi sırasında kullanılan fonksiyon
    pub fn submit(&mut self) {
        if let Some(s) = self.state.take() {
            // O anki trait kimse onun üstünden asıl durum nesnesinin uyguladığı commit fonksiyonu çağırılır.
            self.state = Some(s.commit())
        }
    }
}
