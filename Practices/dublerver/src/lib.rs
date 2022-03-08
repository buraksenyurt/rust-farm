pub mod server {
    use log::{error, info};
    use std::io::{Read, Write};
    use std::net::TcpListener;

    pub struct Server {
        pub alias: String,
        pub address: String,
        pub port: u16,
    }

    impl Server {
        pub fn new(alias: String, address: String, port: u16) -> Self {
            Server {
                alias,
                address,
                port,
            }
        }

        pub fn get_address(&self) -> String {
            format!("{}:{}", self.address, self.port)
        }

        pub fn run(self) {
            let listener = TcpListener::bind(&self.get_address());
            info!("{}\t{} aktif ve dinliyor...", &self.alias, &self.get_address());
            match listener {
                Ok(endpoint) => loop {
                    match endpoint.accept() {
                        Ok((mut stream, _adddress)) => {
                            let mut buffer = [0_u8; 512];
                            if let Ok(length) = stream.read(&mut buffer) {
                                let msg = String::from_utf8(buffer[0..length].to_vec());
                                info!("{:?}", msg.unwrap());
                            }
                            match write!(stream, "HTTP/1.1 200 Ok \r\n\r\nPONG!\r\n") {
                                Ok(_) => {}
                                Err(e) => error!("{}", e),
                            }
                        }
                        Err(e) => {
                            error!("{}", e)
                        }
                    }
                },
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
    }
}
