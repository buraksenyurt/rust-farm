use crate::common::*;

/*
   Aşağıdaki kullanımda Non-Consuming Builder Pattern
   uyarlaması söz konusudur.
   Esnek bir kullanıma sahiptir ancak gereksiz allocation'lar da yapar.
*/

#[derive(Debug)]
pub struct WebRequest {
    pub url: String,
    pub method: String,
    pub body: Option<String>,
    pub headers: Vec<(String, String)>,
}

#[derive(Default)]
pub struct WebRequestBuilder {
    pub url: Option<String>,
    pub method: Option<String>,
    pub body: Option<String>,
    pub headers: Vec<(String, String)>,
}

impl WebRequestBuilder {
    pub fn new() -> Self {
        WebRequestBuilder::default()
    }
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }
    pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
        self.method = Some(method.into());
        self
    }
    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }
    pub fn header(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    // Dikkat: &mut self kullanılmamalı.
    pub fn build(&self) -> Result<WebRequest> {
        let Some(url)=self.url.as_ref() else {return Err(Error::Generic("There is No Url"));};
        let method = self
            .method
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "GET".to_string());
        Ok(WebRequest {
            url: url.to_string(),
            method,
            headers: self.headers.clone(),
            body: self.body.clone(),
        })
    }
}
