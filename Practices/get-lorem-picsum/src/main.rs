use crate::builder::{get_photos, write_to_file};

mod builder;
mod photo;
mod test;

#[tokio::main]
async fn main() {
    let get_photos_result = get_photos(1, 10).await;
    match get_photos_result {
        Ok(photos) => {
            for photo in photos.into_iter() {
                println!("download url -> {}", &photo.download_url);
                let write_result = write_to_file(&photo).await;
                match write_result {
                    Ok(_) => println!("\t{} başarılı bir şekilde oluşturuldu", &photo.url),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
