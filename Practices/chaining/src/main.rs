use crate::concrete_handlers::*;
use crate::handler::{HttpRequest, HttpRequestHandler};

mod concrete_handlers;
mod handler;

fn main() {
    let data_process_handler = DataProcessHandler::default();
    let cache_handler = CacheHandler::default().next(Box::new(data_process_handler));
    let auth_handler = AuthHandler::default().next(Box::new(cache_handler));

    let a_simple_request = HttpRequest {
        headers: vec![("Authentication".to_string(), "Bearer 12345".to_string())],
        path: "/api/data/products/".to_string(),
        body: "{'category':3}".to_string(),
    };

    if let Some(response) = auth_handler.process(&a_simple_request) {
        println!("Response is {} - {}", response.code, response.body);
    } else {
        println!("No response");
    }
}
