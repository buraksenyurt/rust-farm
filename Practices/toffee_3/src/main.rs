//use std::fmt::{Display, Formatter};
use thiserror::Error;

// non_exhaustive // Gelecekte aşağıdaki enum türüne yeni alanlar eklenebileceğini ifade eder.
// Geriye uyumluluk adına kullanılması tavsiye edilir.

/*
#[derive(Debug)]
#[non_exhaustive]
pub enum CommError {
    NotConnect(u8),
    MissingSatellite(String),
    SunStorm,
}*/

#[derive(Debug, Error)]
pub enum CommError {
    #[error("{0} sürede bağlantı sağlanamadı")]
    NotConnect(u8),
    #[error("{0} uydusu mevcut değil")]
    MissingSatellite(String),
    #[error("Güneş fırtınası sebebiyle iletişim hatası")]
    SunStorm,
}

/*
impl Error for CommError {}

impl Display for CommError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use CommError::*;

        match self {
            NotConnect(t) => write!(f, "{} içerisinde bağlantı sağlanamadı.", t),
            MissingSatellite(h) => write!(f, "{}, geçerli bir uydu değil.", h),
            SunStorm => write!(f, "Güneş fırtınası nedeniyle iletişim arızası"),
        }
    }
}
*/

fn main() {
    let status = connect(String::from("tokyo#1234")).unwrap();
    println!("Tokyo#1234 sunucusu ile bağlantı {}", status);

    let status = connect(String::from("yucin#5151"));
    match status {
        Ok(_) => println!("Bağlantı aktif"),
        Err(e) => println!("{:?}", e),
    };

    println!("İletişim Sonu");
}

fn connect(satellite_name: String) -> Result<bool, CommError> {
    if satellite_name.contains("yucin") {
        Err(CommError::MissingSatellite(satellite_name))
    } else {
        Ok(true)
    }
}
