pub struct HttpRequest {
    pub headers: Vec<(String, String)>,
    pub path: String,
    pub body: String,
}
pub struct HttpResponse {
    pub code: u16,
    pub body: String,
}
pub trait HttpRequestHandler {
    fn next(&mut self, handler: Box<dyn HttpRequestHandler>);
    fn process(&self, request: &HttpRequest) -> Option<HttpResponse>;
}
