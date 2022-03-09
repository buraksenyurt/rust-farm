use clap::{Arg, Command};

fn main() {
    let app = Command::new("Crayz Server")
        .about("Tek çekirdek veya çok çekirdek destekli Web sunucusu.")
        .version("v0.1.0")
        .author("Burak Selim Senyurt")
        .subcommand(
            Command::new("start")
                .about("Web sunucusunu başlatır.")
                .short_flag('s')
                .subcommand(
                    Command::new("mode")
                        .about("Hangi mod? Tek çekirdek ya da daha fazlası ;)")
                        .short_flag('m')
                        .args(&[
                            Arg::new("one").short('o').help("Tek thread ile başlar."),
                            Arg::new("parallel")
                                .short('p')
                                .help("Çekirdek sayısı kadar thread ile başlar."),
                        ]),
                ),
        );

    let matches = app.get_matches();
    println!("{:#?}", matches);
}
