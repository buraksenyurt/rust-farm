use crate::common::*;

/*
   Aşağıdaki kullanımda Consuming Builder Pattern uyarlaması söz konusudur.
   Gereksiz allocation'lar da kaçınılmasını sağlayan Ownership kurallarına daha yakın
   bir kalıptır.
*/

#[allow(dead_code)]
#[derive(Debug)]
pub struct HttpRequest {
    url: String,
    method: String,
    body: Option<String>,
    headers: Vec<(String, String)>,
}

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct HttpRequestBuilder {
    url: Option<String>,
    method: Option<String>,
    body: Option<String>,
    headers: Vec<(String, String)>,
}

impl HttpRequestBuilder {
    pub fn new() -> Self {
        HttpRequestBuilder::default()
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<HttpRequest> {
        let Some(url)=self.url else {return Err(Error::Generic("There is No Url"));};
        let method = self.method.unwrap_or_else(|| "GET".to_string());
        Ok(HttpRequest {
            url,
            method,
            headers: self.headers,
            body: self.body,
        })
    }
}
