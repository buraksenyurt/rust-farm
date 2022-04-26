use image::ImageError;
use std::{fmt, io};

#[derive(Debug, PartialEq)]
pub enum ImagixError {
    FileIO(String),
    UserInput(String),
    ImageResizing(String),
    Format(String),
}

impl From<io::Error> for ImagixError {
    fn from(_: io::Error) -> Self {
        ImagixError::FileIO("I/O işlemi sırasında hata".to_string())
    }
}

impl From<ImageError> for ImagixError {
    fn from(_: ImageError) -> Self {
        ImagixError::ImageResizing("Resim dönüştürme işlemi sırasında hata".to_string())
    }
}

impl From<io::ErrorKind> for ImagixError {
    fn from(_: io::ErrorKind) -> Self {
        ImagixError::UserInput("Hatalı girdi".to_string())
    }
}

impl fmt::Display for ImagixError {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            out,
            "{}",
            ImagixError::Format("Bir hata oluştu.".to_string())
        )
    }
}
