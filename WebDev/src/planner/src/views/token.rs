use actix_web::dev::ServiceRequest;

/*
   Taleplerde yetki kontrolü için token mekanizmasından yararlanılabilir.
   Token tarafı bir takım yardımcı fonksiyonlar bu modülde yer almakta.
   Amaç talep header'ından gelen token bilgisinin çekilmesi ve geçerliliğinin kontrolü.
   Sadece geçerli bir token varsa servis operasyonlarının yürütülmesi mümkün olmalı.
*/
fn is_valid_token(token: String) -> Result<String, String> {
    // token kontrolü için ilerki bölümlerde farklı bir yol izlenecek.
    // Büyük ihtimalle ve Identity Server üstünden üretilen token bilgisi ile iletişim sağlanacak.
    if token == "abrakadabra" {
        return Ok(token);
    }
    Err("Token doğrulanamadı".to_string())
}

fn get_header_token(request: &ServiceRequest) -> Result<String, String> {
    // headers içinden user-token alanı aranır
    match request.headers().get("user-token") {
        // eşleşme varsa
        Some(token) => {
            // string karşılığına bakılır
            match token.to_str() {
                Ok(p) => Ok(String::from(p)),
                Err(_) => Err("Token alımı sırasında hata".to_string()),
            }
        }
        None => Err("Token bilgisi bulunamadı".to_string()),
    }
}

// Modül dışına açık olan bu fonksiyon ile user-token bilgisini alıp geçerliliğini kontrol ediyoruz.
pub fn process_token(request: &ServiceRequest) -> Result<String, String> {
    match get_header_token(request) {
        Ok(token) => match is_valid_token(token) {
            Ok(token) => Ok(token),
            Err(message) => Err(message),
        },
        Err(message) => Err(message),
    }
}
