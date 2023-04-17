use crate::builder::get_photos;
use crate::photo::Photo;

mod builder;
mod photo;

#[tokio::main]
async fn main() {
    let photos: Vec<Photo> = get_photos().await.expect("Fotoğraflar çekilemedi.");
    for photo in photos.iter() {
        println!("download url -> {}", photo.download_url);
    }
}
