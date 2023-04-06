/*
Kobay bir iş modülümüz olduğunu düşünelim.
İçindeki fonksiyon harici bir servis çağrısı yapıyor ve servisten gelen cevaba göre
süreci şekillendiriyor.

Bunun testini yazarken servis sanki varmış gibi hareket edip fonksiyonu cover etmek istiyoruz.

İlk önce mockito crate'ini kullanarak işe başladım ancak server context'ini bir türlü
do_accounting fonksiyonu içerisine taşıyamadım. Bunun üzerine alternatif bir mock server aradım
ve wiremock'ta karar kıldım. Kütür kütür çalıştı :D

Örnekte olmayan servise çağrı yapmak için hyper paketinden yararlanıyoruz. Bu olmayan servise
gelen çağrıları mocklamak içinse wiremock'tan faydalanıyorum.

Şimdilik tek sıkıntı do_accounting'e wiremock server adresini de göndermek zorunda kalmam.
Nitekim her seferinde farklı bir porttan bağlanıyor.
*/

mod business {
    use hyper::{Body, Client, Method, Request, StatusCode};
    use serde::{Deserialize, Serialize};

    // Url'i taşımadan mock'lamanın bir yolunu bulmam lazım.
    pub async fn do_accounting(
        customer: &Customer,
        url: String,
    ) -> Result<BusinessResponse, Box<dyn std::error::Error + Send + Sync>> {
        //println!("Talep yapılacak adres {}", url);
        let body = serde_json::to_string(customer).unwrap();
        let request = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header("content-type", "application/json")
            .body(Body::from(body))?;

        let client = Client::new();
        let response = client.request(request).await?;
        //println!("{:?}", response.status());
        if response.status() == StatusCode::OK {
            // Limit müsaitse bazı işlemler yapılacak
            let body = hyper::body::to_bytes(response.into_body()).await?;
            let result: DoAccountingResponse = serde_json::from_slice(&body)?;
            if result.code == 1 {
                return Ok(BusinessResponse::new(
                    ReturnCode::Success,
                    "İşleme uygun.".to_string(),
                ));
            }
            Ok(BusinessResponse::new(
                ReturnCode::Unsufficient,
                "Limit yetersiz!".to_string(),
            ))
        } else {
            Ok(BusinessResponse::new(
                ReturnCode::UnknownError,
                "Bilinmeyen hata!".to_string(),
            ))
        }
    }

    #[derive(Serialize)]
    pub struct Customer {
        pub id: i32,
        pub title: String,
        pub balance: f32,
    }

    impl Customer {
        pub fn new(id: i32, title: String, balance: f32) -> Self {
            Self { id, title, balance }
        }
    }

    #[derive(Deserialize)]
    pub struct DoAccountingResponse {
        pub code: i32,
        pub message: String,
    }

    pub struct BusinessResponse {
        pub return_code: ReturnCode,
        pub message: String,
    }

    #[derive(Debug, PartialEq)]
    pub enum ReturnCode {
        Success,
        Unsufficient,
        UnknownError,
    }

    impl BusinessResponse {
        pub fn new(return_code: ReturnCode, message: String) -> Self {
            Self {
                return_code,
                message,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::business::{do_accounting, Customer, DoAccountingResponse, ReturnCode};
    use serde_json::json;
    use surf::StatusCode;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn should_wiremock_works() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/bank/api/checkLimit"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "code": 1, "message": "Limit uygun." })),
            )
            .mount(&server)
            .await;

        let mut response = surf::post(format!("{}/bank/api/checkLimit", &server.uri()))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::Ok);
        let body: DoAccountingResponse = response.body_json().await.unwrap();
        assert_eq!(body.code, 1);
        assert_eq!(body.message, "Limit uygun.");
    }

    #[tokio::test] // asenkron fonksiyonları test etmek için kullanılır
                   //#[should_panic]
    async fn should_customer_limit_sufficent() {
        let server = MockServer::start().await;
        let url = format!(
            "http://{}/bank/api/checkLimit",
            server.address().to_string()
        );
        Mock::given(method("POST"))
            .and(path("/bank/api/checkLimit"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "code": 1, "message": "Limit uygun." })),
            )
            .mount(&server)
            .await;

        let cust = Customer::new(1230, "Sir Connery".to_string(), 1000.00);
        let accounting_result = do_accounting(&cust, url).await.unwrap();
        assert_eq!(accounting_result.return_code, ReturnCode::Success);
    }

    #[tokio::test] // asenkron fonksiyonları test etmek için kullanılır
                   //#[should_panic]
    async fn should_customer_limit_unsufficent() {
        let server = MockServer::start().await;
        let url = format!(
            "http://{}/bank/api/checkLimit",
            server.address().to_string()
        );
        Mock::given(method("POST"))
            .and(path("/bank/api/checkLimit"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "code": 0, "message": "Limit yetersiz!" })),
            )
            .mount(&server)
            .await;

        let cust = Customer::new(1230, "Sir Connery".to_string(), 150.00);
        let accounting_result = do_accounting(&cust, url).await.unwrap();
        assert_eq!(accounting_result.return_code, ReturnCode::Unsufficient);
    }
}
