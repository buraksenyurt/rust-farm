use clap::{arg, Command};
use clappy::Format;

fn main() {
    // #1 Örnek
    // let matches = Command::new("Crazy Server")
    //     .propagate_version(true)
    //     .subcommand_required(true)
    //     .arg_required_else_help(true)
    //     .about("Tek çekirdek veya çok çekirdek destekli Web sunucusu.")
    //     .version("v0.1.0")
    //     .author("Burak Selim Senyurt")
    //     .subcommand(
    //         Command::new("run")
    //             .about("Web sunucusunu başlatır.")
    //             .short_flag('r')
    //             .arg(
    //                 arg!([MODE] "Hangi modda çalıştırmak istersin?")
    //                     .possible_values(["basic", "advanced"]),
    //             ),
    //     )
    //     .get_matches();
    //
    // match matches.subcommand() {
    //     Some(("run", argmatchs)) => {
    //         let mode = argmatchs
    //             .value_of("MODE")
    //             .expect("Bir mod girilmemiş olabilir.");
    //         //println!("{} moduna göre işlem yapılacak.", mode);
    //         match mode {
    //             "basic" => println!("Tek thread ile başlatılacak."),
    //             "advanced" => println!("Çekirdek sayısı kadar thread açılacak"),
    //             _ => unreachable!(),
    //         }
    //     }
    //     _ => {}
    // }

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

    // #3 Örnek
    let matches = Command::new("Crazy Server")
        .arg_required_else_help(true)
        .about("Tek çekirdek veya çok çekirdek destekli Web sunucusu.")
        .version("v0.1.0")
        .author("Burak Selim Senyurt")
        .subcommand(
            Command::new("convert")
                .about("Dönüştürme işlemi için")
                .short_flag('c')
                .arg(
                    arg!(<FORMAT>)
                        .help("Hangi formatı kullanmak istersin?")
                        .possible_values(Format::possible_values()),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("convert", argmatchs)) => {
            match argmatchs.value_of_t("FORMAT").expect("Argüman hatası") {
                Format::Json => {
                    println!("JSON serileştirme yapılacak");
                }
                Format::Bson => {
                    println!("Binary JSON serileştirme yapılacak");
                }
                Format::Xml => {
                    println!("XML serileştirme yapılacak");
                }
                Format::Binary => {
                    println!("Binary sıkıştırma yapılacak");
                }
            }
        }
        _ => unreachable!()
    }
}
