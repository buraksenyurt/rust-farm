fn main() {
    println!("Hello, world!");
}

struct HttpRequest {
    headers: Vec<(String, String)>,
    path: String,
    body: String,
}
struct HttpResponse {
    code: u16,
    body: String,
}
trait HttpRequestHandler {
    fn next(&mut self, handler: Box<dyn HttpRequestHandler>);
    fn process(&self, request: &HttpRequest) -> HttpResponse;
}
