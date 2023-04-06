/*
Kobay bir iş modülümüz olduğunu düşünelim.
İçindeki fonksiyon harici bir servis çağrısı yapıyor ve servisten gelen cevaba göre
süreci şekillendiriyor.

Bunun testini yazarken servis sanki varmış gibi hareket edip fonksiyonu cover etmek istiyoruz.
*/

mod business {
    use hyper::{Body, Client, Method, Request, StatusCode};
    use serde::{Deserialize, Serialize};

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    pub async fn do_accounting(customer: &Customer) -> Result<BusinessResponse> {
        let body = serde_json::to_string(customer).unwrap();
        let request = Request::builder()
            .method(Method::POST)
            .uri("http://localhost:7777/bank/api/checkLimit")
            .header("content-type", "application/json")
            .body(Body::from(body))?;

        let client = Client::new();
        let response = client.request(request).await?;
        if response.status() == StatusCode::OK {
            // Limit müsaitse bazı işlemler yapılacak
            let body = hyper::body::to_bytes(response.into_body()).await?;
            let result: DoAccountingResponse = serde_json::from_slice(&body).unwrap();
            if result.code == 1 {
                return Ok(BusinessResponse::new(
                    ReturnCode::Success,
                    "İşleme uygun.".to_string(),
                ));
            }
            Ok(BusinessResponse::new(
                ReturnCode::LimitUnavailable,
                "Limit yetersiz!".to_string(),
            ))
        } else {
            Ok(BusinessResponse::new(
                ReturnCode::UnknownError,
                "Bilinmeye hata!".to_string(),
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
        LimitUnavailable,
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
    use crate::business::{do_accounting, Customer, ReturnCode};

    #[tokio::test] // asenkron fonksiyonları test etmek için kullanılır
    async fn should_do_accounting_works() {
        let cust = Customer::new(1230, "Sir Connery".to_string(), 1000.00);
        let accounting_result = do_accounting(&cust).await.unwrap();
        assert_eq!(accounting_result.return_code, ReturnCode::UnknownError);
    }
}
