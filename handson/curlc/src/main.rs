use crate::io::parser::LineOutput;
use crate::io::reader::read_file;
use crate::transport::http::Method;

mod io;
mod tests;
mod transport;

fn main() {
    let client = reqwest::blocking::Client::new();
    for line in read_file("initial2.req").into_iter() {
        match line {
            LineOutput::Valid(request) => {
                let url = format!("https://{}", request.url);
                let res = match request.method {
                    Method::Post { body } => client.post(&url).body(body),
                    Method::Get => client.get(&url),
                }
                .send()
                .unwrap();
                println!("{}", res.text().unwrap());
            }
            _ => {}
        }
    }
}
