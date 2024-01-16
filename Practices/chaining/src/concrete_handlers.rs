use crate::handler::*;

#[derive(Default)]
pub struct AuthHandler {
    pub next: Option<Box<dyn HttpRequestHandler>>,
}

impl AuthHandler {
    pub fn authenticate(&self, _request: &HttpRequest) -> bool {
        // Burada gerçekten authentication kontrolü yapıldığını düşünelim
        true
    }
}

impl HttpRequestHandler for AuthHandler {
    fn next(&mut self, handler: Box<dyn HttpRequestHandler>) {
        self.next = Some(handler);
    }
    fn process(&self, request: &HttpRequest) -> Option<HttpResponse> {
        if !self.authenticate(request) {
            return Some(HttpResponse {
                code: 401,
                body: "Unauthorized".to_string(),
            });
        }

        match &self.next {
            Some(next) => next.process(request),
            None => Some(HttpResponse {
                code: 200,
                body: "Authenticated".to_string(),
            }),
        }
    }
}

#[derive(Default)]
pub struct CacheHandler {
    pub next: Option<Box<dyn HttpRequestHandler>>,
}

impl CacheHandler {
    pub fn set(&self, _request: &HttpRequest) {
        // Burada gerçekten mesaj içeriği ile ilgili bir cache' leme yapıldığını düşünelim
    }
}

impl HttpRequestHandler for CacheHandler {
    fn next(&mut self, handler: Box<dyn HttpRequestHandler>) {
        self.next = Some(handler);
    }
    fn process(&self, request: &HttpRequest) -> Option<HttpResponse> {
        self.set(request);

        match &self.next {
            Some(next) => next.process(request),
            None => None,
        }
    }
}

#[derive(Default)]
pub struct DataProcessHandler {
    pub next: Option<Box<dyn HttpRequestHandler>>,
}

impl DataProcessHandler {
    pub fn do_somethings(&self, _request: &HttpRequest) {
        // Burada mesaj içeriğine göre bir takım işler gerçekleştirildiğini düşünelim
    }
}

impl HttpRequestHandler for DataProcessHandler {
    fn next(&mut self, handler: Box<dyn HttpRequestHandler>) {
        self.next = Some(handler);
    }
    fn process(&self, request: &HttpRequest) -> Option<HttpResponse> {
        self.do_somethings(request);

        match &self.next {
            Some(next) => next.process(request),
            None => Some(HttpResponse {
                code: 200,
                body: "Processed".to_string(),
            }),
        }
    }
}
