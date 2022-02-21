use crayz_server::server::Server;

fn main() {
    // loglamayı açtık
    env_logger::init();

    // Server veri yapımızı kullanarak bir örnek oluşturduk
    let alpha = Server::new("0.0.0.0", 5555_u16, "localhost");
    // run fonksiyonunu çağırıp sunucuyu başlatıyoruz. ya da başlatamıyoruz :)
    alpha.run();
}
