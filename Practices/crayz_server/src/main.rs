use crayz_server::{Method, Request, Server};
use log::info;

fn main() {
    env_logger::init();

    let alpha = Server::new("127.0.0.1".to_string(), 8080_u16, "localhost".to_string());
    alpha.run();

    let get_player = Request {
        method: Method::Get(Some("?layer>50".to_string())),
        path: "/player".to_string(),
    };

    info!("{}", get_player);
}
