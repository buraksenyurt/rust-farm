/*
Kobay bir iş modülümüz olduğunu düşünelim.
İçindeki fonksiyon harici bir servis çağrısı yapıyor ve servisten gelen cevaba göre
süreci şekillendiriyor.

Bunun testini yazarken servis sanki varmış gibi hareket edip fonksiyonu cover etmek istiyoruz.
*/
mod business {
    use hyper::{Body, Client, Method, Request, StatusCode};
    use serde::Serialize;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    pub async fn do_accounting(customer: &Customer) -> Result<bool> {
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
            Ok(true)
        } else {
            // Limit yetersiz gibi bir hata dönülebilir
            Ok(false)
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
}

#[cfg(test)]
mod tests {
    use crate::business::{do_accounting, Customer};

    #[tokio::test]
    async fn should_do_accounting_works() {
        let cust = Customer::new(1230, "Sir Connery".to_string(), 1000.00);
        let accounting_result = do_accounting(&cust).await.unwrap();
        assert_eq!(accounting_result, false);
    }
}
