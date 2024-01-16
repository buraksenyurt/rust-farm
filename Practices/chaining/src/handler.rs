pub struct HttpRequest {
    pub headers: Vec<(String, String)>,
    pub path: String,
    pub body: String,
}
#[derive(Copy, Clone)]
pub struct HttpResponse<'a> {
    pub code: u16,
    pub body: &'a str,
}
pub trait HttpRequestHandler {
    //fn set_next(&mut self, handler: Box<dyn HttpRequestHandler>);
    fn process(&self, request: &HttpRequest) -> Option<HttpResponse>;
}
