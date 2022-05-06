use crate::command_line::CommandLine;
use imagix::resize::process_resize_request;
use imagix::stats::get_stats;
use structopt::StructOpt;

mod command_line;

fn main() {
    let args = CommandLine::from_args();
    match args {
        CommandLine::Resize {
            size,
            mode,
            mut sourcefolder,
        } => match process_resize_request(size, mode, &mut sourcefolder) {
            Ok(_) => println!("Resim(ler) yeniden boyutlandırıldı"),
            Err(e) => println!("Bir hata söz konusu {}", e),
        },
        CommandLine::Stats { sourcefolder } => match get_stats(sourcefolder) {
            Ok(stats) => println!("{}", stats),
            Err(e) => println!("İstatistik alımında hata {}", e),
        },
    }
}
