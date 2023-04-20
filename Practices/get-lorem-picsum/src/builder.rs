use crate::argument::List;
use crate::photo::Photo;
use reqwest::StatusCode;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

#[derive(Debug, PartialEq)]
pub enum ProcessError {
    FileCreate,
    OverLimit,
    SendError,
    UnreadBytes,
    Unsuccessful,
    WriteError,
}

pub async fn download_many(list: List) {
    let get_photos_result = get_photos(list.page_number, list.count).await;
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

pub async fn download_single(photo_id: u32) {
    let photo_info_result = get_photo_info(PhotoKind::Single(photo_id)).await;
    match photo_info_result {
        Ok(photo) => {
            println!("download url -> {}", &photo.download_url);
            let write_result = write_to_file(&photo).await;
            match write_result {
                Ok(_) => println!("\t{} başarılı bir şekilde oluşturuldu", &photo.url),
                Err(e) => println!("{:?}", e),
            }
        }
        Err(e) => println!("{:?}", e),
    }
}

pub async fn download_random() {
    let photo_info_result = get_photo_info(PhotoKind::Random).await;
    match photo_info_result {
        Ok(photo) => {
            println!("download url -> {}", &photo.download_url);
            let write_result = write_to_file(&photo).await;
            match write_result {
                Ok(_) => println!("\t{} başarılı bir şekilde oluşturuldu", &photo.url),
                Err(e) => println!("{:?}", e),
            }
        }
        Err(e) => println!("{:?}", e),
    }
}

enum PhotoKind {
    Random,
    Single(u32),
}
async fn get_photo_info(kind: PhotoKind) -> Result<Photo, ProcessError> {
    let url;
    match kind {
        PhotoKind::Single(photo_id) => url = format!("https://picsum.photos/id/{}/info", photo_id),
        PhotoKind::Random => {
            let response = reqwest::Client::new()
                .get("https://picsum.photos/200/300".to_string())
                .header("User-Agent", "Reqwest Client")
                .send()
                .await;
            match response {
                Ok(r) => {
                    url = format!(
                        "https://picsum.photos/id/{}/info",
                        r.headers().get("Picsum-Id").unwrap().to_str().unwrap()
                    )
                }
                _ => {
                    println!("Rastgele foto çağrısında hata");
                    return Err(ProcessError::Unsuccessful);
                }
            }
        }
    }
    let response = reqwest::Client::new()
        .get(url)
        .header("User-Agent", "Reqwest Client")
        .send()
        .await;

    match response {
        Ok(r) => match r.status() {
            StatusCode::OK => match r.json::<Photo>().await {
                Ok(parsed) => Ok(parsed),
                Err(e) => {
                    println!("{}", e);
                    Err(ProcessError::Unsuccessful)
                }
            },
            _ => {
                println!("Status Code uygun değil");
                Err(ProcessError::Unsuccessful)
            }
        },
        Err(e) => {
            println!("{}", e);
            Err(ProcessError::Unsuccessful)
        }
    }
}

async fn get_photos(page: u8, limit: u8) -> Result<Vec<Photo>, ProcessError> {
    if page <= 0 || limit > 25 {
        return Err(ProcessError::OverLimit);
    }
    let response = reqwest::Client::new()
        .get(format!(
            "https://picsum.photos/v2/list?page={}&limit={}",
            page, limit
        ))
        .header("User-Agent", "Reqwest Client")
        .send()
        .await;

    match response {
        Ok(r) => match r.status() {
            StatusCode::OK => match r.json::<Vec<Photo>>().await {
                Ok(parsed) => Ok(parsed),
                Err(e) => {
                    println!("{}", e);
                    Err(ProcessError::Unsuccessful)
                }
            },
            _ => {
                println!("Status Code uygun değil");
                Err(ProcessError::Unsuccessful)
            }
        },
        Err(e) => {
            println!("{}", e);
            Err(ProcessError::Unsuccessful)
        }
    }
}

async fn write_to_file(photo: &Photo) -> Result<bool, ProcessError> {
    let file_name = photo.create_file_name();
    let file_create = File::create(format!("./Photos/{}", file_name));
    if file_create.is_err() {
        return Err(ProcessError::FileCreate);
    }
    let send_result = reqwest::Client::new()
        .get(&photo.download_url)
        .header("User-Agent", "Reqwest Client")
        .send()
        .await;
    if send_result.is_err() {
        return Err(ProcessError::SendError);
    }
    let content_result = send_result.unwrap().bytes().await;
    if content_result.is_err() {
        return Err(ProcessError::UnreadBytes);
    }
    let content = content_result.unwrap();
    let mut file = file_create.unwrap();
    let mut pos = 0;
    while pos < content.len() {
        let bytes_written = file.write(&content[pos..]);
        if bytes_written.is_err() {
            return Err(ProcessError::WriteError);
        }
        pos += bytes_written.unwrap();
    }
    Ok(true)
}

pub fn check_and_create_folder() -> bool {
    if !Path::new("Photos").exists() {
        if !fs::create_dir("Photos").is_ok() {
            return false;
        }
    }
    true
}

pub fn show_help() {
    let help_text = "
            Parametre sayısı hatalı!

            Örnek Kullanımlar;

            -- Belli bir ID için
            get-lorem-picsum single 123

            -- 3ncü sayfadan itibaren 10 kayıt için
            get-lorem-picsum many 3 10
            ";
    println!("{}", help_text);
    exit(1);
}
