use clap::Parser;
use rexif::ExifTag;

use crate::cli::Cli;

mod cli;
fn main() {
    let args = Cli::parse();

    match rexif::parse_file(&args.image_path) {
        Ok(exif_data) => {
            println!("Exif mime {}\n", exif_data.mime);

            for entry in &exif_data.entries {
                println!("{}: {:?}", entry.tag, entry.value_more_readable);
            }

            println!("\nSelected EXIF Tags:");

            let tags = [
                ExifTag::Model,
                ExifTag::DateTimeOriginal,
                ExifTag::Orientation,
                ExifTag::XResolution,
                ExifTag::YResolution,
                ExifTag::ResolutionUnit,
                ExifTag::ExposureTime,
                ExifTag::FNumber,
                ExifTag::ISOSpeedRatings,
                ExifTag::FocalLength,
            ];
            for tag in tags.iter() {
                if let Some(entry) = exif_data.entries.iter().find(|e| &e.tag == tag) {
                    println!("{}: {:?}", entry.tag, entry.value_more_readable.trim());
                }
            }
        }
        Err(e) => eprintln!("Error reading EXIF data: {}", e),
    }
}
