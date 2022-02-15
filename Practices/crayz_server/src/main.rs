// use crayz_server::http::common::Method;
// use crayz_server::http::request::Request;
use crayz_server::server::Server;

fn main() {
    // loglamayı açtık
    env_logger::init();

    // Server veri yapımızı kullanarak bir örnek oluşturduk
    let alpha = Server::new("0.0.0.0".to_string(), 5555_u16, "localhost".to_string());
    // run fonksiyonunu çağırıp sunucuyu başlatıyoruz. ya da başlatamıyoruz :)
    alpha.run();


    // let get_player = Request {
    //     method: Method::Get(Some("?layer>50".to_string())),
    //     path: "/player".to_string(),
    // };
    //
    // info!("{}", get_player);
}
