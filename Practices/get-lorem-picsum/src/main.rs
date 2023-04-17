use crate::builder::{get_photos, write_to_file};
use crate::photo::Photo;

mod builder;
mod photo;

#[tokio::main]
async fn main() {
    let photos: Vec<Photo> = get_photos().await.expect("Fotoğraflar çekilemedi.");
    for (i, photo) in photos.into_iter().enumerate() {
        println!("download url -> {}", photo.download_url);
        write_to_file(photo.download_url, format!("photo_{}.jpg", i)).await;
    }
}
