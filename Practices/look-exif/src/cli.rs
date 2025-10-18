use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    author = "Burak Selim Senyurt",
    about = "A experimental CLI tool to view and manipulate EXIF metadata in images."
)]
pub struct Cli {
    #[arg(short = 'i', long = "image", help = "Path to the image file")]
    pub image_path: String,
}
