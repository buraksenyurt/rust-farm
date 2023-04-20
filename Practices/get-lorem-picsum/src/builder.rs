use crate::argument::List;
use crate::photo::Photo;
use reqwest::StatusCode;
use std::fs::File;
use std::io::Write;

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
