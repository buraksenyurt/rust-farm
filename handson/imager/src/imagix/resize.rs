use crate::error::ImagixError;
use std::path::PathBuf;
use std::{fs, io};

pub fn process_resize_request() {
    todo!()
}

fn resize_single() {
    todo!()
}

fn resise_all() {
    todo!()
}

fn resize_images() {
    todo!()
}

/// Kaynak klasördeki jpg ve png dosyalarının listesini verir
fn get_image_files(source: PathBuf) -> Result<Vec<PathBuf>, ImagixError> {
    // source parametresi ile gelen dosya yolunu alıp içeriğinde dolaşıyoruz.
    // Geçersi bir klasör olma durumunu map_err ile kontrol altına alıyoruz. ? kullanımına dikkat.
    let files = fs::read_dir(source)
        .map_err(|_e| ImagixError::UserInput("Geçersiz klasör".to_string()))?
        .map(|resource| resource.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .into_iter()
        .filter(|r| {
            r.extension() == Some("JPG".as_ref())
                || r.extension() == Some("jpg".as_ref())
                || r.extension() == Some("PNG".as_ref())
                || r.extension() == Some("png".as_ref())
        })
        .collect();
    Ok(files)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_image_files_works_test() {
        let source_directory = PathBuf::from("./images/");
        let result = get_image_files(source_directory);
        match result {
            Ok(entries) => assert_eq!(entries.is_empty(), false),
            Err(_) => {}
        }
    }

    #[test]
    fn get_image_files_fails_test() {
        let source_directory = PathBuf::from("./none/");
        let result = get_image_files(source_directory);
        match result {
            Ok(_) => {}
            Err(e) => {
                assert_eq!(e, ImagixError::UserInput("Geçersiz klasör".to_string()))
            }
        }
    }
}
