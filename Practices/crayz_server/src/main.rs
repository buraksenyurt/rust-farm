use crayz_server::{Method, Request, Server};

fn main() {
    let alpha = Server::new("127.0.0.1".to_string(), 8080_u16, "localhost".to_string());
    alpha.run();

    let get_player = Request {
        method: Method::Get("?layer>50".to_string()),
        path: "/player".to_string(),
    };

    print!("{}", get_player);
}
