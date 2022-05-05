use crate::error::ImagixError;
use crate::resize::get_image_files;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

/// Klasör içindeki imgelere ait istatistikleri verir. Klasör, toplam boyut ve dosya sayısı
pub fn get_stats(source: PathBuf) -> Result<Statistics, ImagixError> {
    let image_files = get_image_files(source.to_path_buf())?;
    let size = image_files
        .iter()
        .map(move |f| f.metadata().unwrap().len())
        .sum::<u64>();
    let statistics = Statistics::new(source, image_files.len(), size as f64);
    Ok(statistics)
}

pub struct Statistics {
    pub folder: PathBuf,
    pub total_files: usize,
    pub total_size: f64,
}

impl Statistics {
    pub fn new(folder: PathBuf, total_files: usize, total_size: f64) -> Self {
        Self {
            folder,
            total_files,
            total_size,
        }
    }
}

impl Display for Statistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} klasöründe {} adet dosya var. Toplam boyut {}",
            self.folder, self.total_files, self.total_size
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_stats_for_specific_path_test() {
        let path = PathBuf::from("./images");
        let statistics = get_stats(path).unwrap();
        assert_eq!(statistics.total_files, 5);
        assert_eq!(statistics.folder.to_str().unwrap(), "./images");
        assert!(statistics.total_size > 327.0 * 1_000.0);
    }
}
