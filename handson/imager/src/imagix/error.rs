use std::{fmt, io};

pub enum ImagixError {
    FileIO(String),
    UserInput(String),
    ImageResizing(String),
    Format(String),
}

impl From<io::Error> for ImagixError {
    fn from(_error: io::Error) -> Self {
        ImagixError::FileIO("I/O işlemi sırasında hata".to_string())
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
