use dublerver::server::Server;
use std::thread;

fn main() {
    env_logger::init();

    let mut handlers = Vec::new();
    handlers.push(thread::spawn(|| {
        let query_server = Server::new("Q-SRV".to_string(), "0.0.0.0".to_string(), 5555_u16);
        query_server.run();
    }));

    handlers.push(thread::spawn(|| {
        let command_server = Server::new("C-SRV".to_string(), "0.0.0.0".to_string(), 5556_u16);
        command_server.run();
    }));

    for handle in handlers {
        let _ = handle.join();
    }
}
