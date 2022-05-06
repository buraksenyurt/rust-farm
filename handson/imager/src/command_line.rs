use imagix::{mode::Mode, size::Size};
use std::path::PathBuf;
use structopt::StructOpt;

/*
    Terminalden alacağımı parametrelerin yönetimi için kullandığımız enum veri yapısı.
    structopt paketi ile gelen makroları kullanarak gerekli bilgileri veriyoruz.
    Program adı, hakkında, yardım gibi bilgiler haricinde,
    parametreleri de ayrıca tanımlayabiliyoruz. Parametrelerde özellikle enum sabiti kullanılan
    hallerde terminalden String olarak gelen içeriğin dönüştürülebilmesi için FromStr trait'inin
    uygulanmış olması gerekiyor.
 */
#[derive(StructOpt, Debug)]
#[structopt(
    name = "resize",
    about = "Resimlerini yeniden boyutlandırmanızı kolaylaştıran bir terminal aracı",
    help = "Yardım için imager resize --help ya da imager stats --help yazın."
)]
pub enum CommandLine {
    #[structopt(help = "Boyutlar(small/medium/large), Modlar(single/full) ve sourcefolder")]
    Resize {
        #[structopt(long)]
        size: Size,
        #[structopt(long)]
        mode: Mode,
        #[structopt(long, parse(from_os_str))]
        sourcefolder: PathBuf,
    },
    #[structopt(help = "İstatistik almak için kaynak klasörü belirtin")]
    Stats {
        #[structopt(long, parse(from_os_str))]
        sourcefolder: PathBuf,
    },
}
