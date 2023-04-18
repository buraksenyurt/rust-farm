use crate::builder::{get_photos, write_to_file};
use crate::photo::Photo;

mod builder;
mod photo;
mod test;

#[tokio::main]
async fn main() {
    let photos: Vec<Photo> = get_photos().await.expect("Fotoğraflar çekilemedi.");
    for photo in photos.into_iter() {
        println!("download url -> {}", &photo.download_url);
        let write_result = write_to_file(&photo).await;
        match write_result {
            Ok(_) => println!("\t{} başarılı bir şekilde oluşturuldu", &photo.url),
            Err(e) => println!("{:?}", e),
        }
    }
}
