fn main() {
    let binding = Binding::Https(SoapVersion::V1_2);
    prepare_env(binding);
    prepare_env(binding);
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum SoapVersion {
    V1_1,
    V1_2,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Binding {
    Http(SoapVersion),
    Https(SoapVersion),
    Rest,
    Grpc,
}

fn prepare_env(b: Binding) {
    match b {
        Binding::Http(v) | Binding::Https(v) => {
            println!("HTTP {:?} versiyonu için ortam hazırlanıyor.", v)
        }
        Binding::Rest => println!("Servis REST protokolüne göre hazırlanıyor"),
        Binding::Grpc => println!("Servis GRPC protoklüne göre hazırlanıyor"),
    }
}
