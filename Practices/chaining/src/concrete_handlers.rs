use crate::handler::*;

#[derive(Default)]
pub struct AuthHandler {
    pub next: Option<Box<dyn HttpRequestHandler>>,
}

impl AuthHandler {
    pub fn next(mut self, handler: Box<dyn HttpRequestHandler>) -> Self {
        self.next = Some(handler);
        self
    }
    pub fn authenticate(&self, _request: &HttpRequest) -> bool {
        println!("Authenticating...");
        true
    }
}

impl HttpRequestHandler for AuthHandler {
    fn process(&self, request: &HttpRequest) -> Option<HttpResponse> {
        if !self.authenticate(request) {
            return Some(HttpResponse {
                code: 401,
                body: "Unauthorized",
            });
        }

        match &self.next {
            Some(next) => next.process(request),
            None => Some(HttpResponse {
                code: 200,
                body: "Authenticated",
            }),
        }
    }
}

#[derive(Default)]
pub struct CacheHandler {
    pub next: Option<Box<dyn HttpRequestHandler>>,
}

impl CacheHandler {
    pub fn next(mut self, handler: Box<dyn HttpRequestHandler>) -> Self {
        self.next = Some(handler);
        self
    }
    pub fn set(&self, _request: &HttpRequest) {
        // Burada gerçekten mesaj içeriği ile ilgili bir cache' leme yapıldığını düşünelim
        println!("Caching...");
    }
}

impl HttpRequestHandler for CacheHandler {
    fn process(&self, request: &HttpRequest) -> Option<HttpResponse> {
        self.set(request);

        match &self.next {
            Some(next) => next.process(request),
            None => Some(HttpResponse {
                code: 200,
                body: "Authenticated",
            }),
        }
    }
}

#[derive(Default)]
pub struct DataProcessHandler {
    pub next: Option<Box<dyn HttpRequestHandler>>,
}

impl DataProcessHandler {
    pub fn next(mut self, handler: Box<dyn HttpRequestHandler>) -> Self {
        self.next = Some(handler);
        self
    }
    pub fn do_somethings(&self, _request: &HttpRequest) {
        // Burada mesaj içeriğine göre bir takım işler gerçekleştirildiğini düşünelim
        println!("Data processing...");
    }
}

impl HttpRequestHandler for DataProcessHandler {
    fn process(&self, request: &HttpRequest) -> Option<HttpResponse> {
        self.do_somethings(request);

        match &self.next {
            Some(next) => next.process(request),
            None => Some(HttpResponse {
                code: 200,
                body: "Processed",
            }),
        }
    }
}
