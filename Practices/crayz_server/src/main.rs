use crayz_server::Server;

fn main() {
    let alpha = Server::new(
        String::from("127.0.0.1"),
        8080_u16,
        String::from("localhost"),
    );
    alpha.run();
}
