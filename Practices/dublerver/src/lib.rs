pub mod server {
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
    }
}
