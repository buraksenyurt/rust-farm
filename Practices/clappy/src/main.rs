use clap::{arg, Command};

fn main() {
    // #1 Örnek
    let matches = Command::new("Crazy Server")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .about("Tek çekirdek veya çok çekirdek destekli Web sunucusu.")
        .version("v0.1.0")
        .author("Burak Selim Senyurt")
        .subcommand(
            Command::new("run")
                .about("Web sunucusunu başlatır.")
                .short_flag('s')
                .arg(arg!([MODE] "Hangi modda çalıştırmak istersin?")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", argmatchs)) => {
            let mode = argmatchs
                .value_of("MODE")
                .expect("Bir mod girilmemiş olabilir.");
            println!("{} moduna göre işlem yapılacak.", mode);
        }
        _ => {}
    }

    // #2 Örnek
    // Bir diğer kullanım şekli
    // let matches = Command::new("Crazy Server")
    //     .version("v0.1.0")
    //     .author("Burak Selim Senyurt")
    //     .about("Tek çekirdek veya çok çekirdek destekli Web sunucusu.")
    //     .arg(arg!(-s --start ... "Sunucuyu başlatmak için kullanılır."))
    //     .arg(arg!(-m --mode <VALUE> "Tek thread veya paralel iki thread.").required(true))
    //     .get_matches();
    //
    // let mode = matches.value_of("mode").unwrap();
    // println!("Gelen mode bilgisi {}", mode);
}
