use crate::error::ImagixError;
use crate::mode::Mode;
use crate::size::Size;
use image::ImageFormat;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{fs, io};

/// Belirtilen mod ve boyuta göre kaynak klasördeki imgeleri küçültür
pub fn process_resize_request(
    size: Size,
    mode: Mode,
    source_folder: &mut PathBuf,
) -> Result<(), ImagixError> {
    let target_size = u32::from(size);
    match mode {
        Mode::Single => resize_single(target_size, target_size, source_folder)?,
        Mode::Full => resize_full(target_size, target_size, source_folder)?,
    };
    Ok(())
}

/// Tek bir dosyayı belirtilen boyutlarda dönüştürmek için kullanılal fonksiyon
fn resize_single(width: u32, height: u32, source: &mut PathBuf) -> Result<(), ImagixError> {
    let source = source;
    resize_image(width, height, source)?;
    Ok(())
}

/// Klasördeki tüm dosyaları parametre olarak gelen genişlik ve yükseklikte küçültür.
fn resize_full(width: u32, height: u32, source: &Path) -> Result<(), ImagixError> {
    if let Ok(files) = get_image_files(source.to_path_buf()) {
        for mut file in files {
            resize_image(width, height, &mut file)?;
        }
    }
    Ok(())
}

/// Kaynak klasördeki jpg ve png dosyalarının listesini verir
pub fn get_image_files(source: PathBuf) -> Result<Vec<PathBuf>, ImagixError> {
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

/// Resmi yeniden boyutlandırmak için kullanılır
fn resize_image(width: u32, height: u32, source_folder: &mut PathBuf) -> Result<(), ImagixError> {
    // kaynak klasör kullanılarak yeni bir dosya ismi oluşturulur. Uzantısı png.
    let target_file_name = source_folder
        .file_stem()
        .unwrap()
        .to_str()
        .ok_or(ErrorKind::InvalidInput)
        .map(|f| format!("{}.png", f));

    // Kaykan klasörden yararlanılarak dönüştürülen dosyaların ekleneceği hedef klasör oluşturulur
    let mut target_folder = source_folder.clone();
    // Çalışma mantığı bir stack veri yapısı gibi. Last in First Out -LIFO
    // pop ile PathBuf'ın son kısmını veri serisinden çıkarılıyor.
    target_folder.pop();
    // temp/ şeklinde yeni bir klasör bilgisi ilave ediliyor.
    target_folder.push("temp/");
    // oluşan yeni klasör bilgisi sistemde yoksa fs modülünün create_dir fonksiyonu ile oluşturulur.
    if !target_folder.exists() {
        fs::create_dir(&target_folder)?;
    }

    // Sırada hedef klasör içindeki yeniden boyulandırılacak dosyanın oluşturulması işlemi var.
    // yine pop ile son eklenen kısmı alıyoruz. Alırken tabii listeden çıkartılıyor. Pop işlemi sonuçta.
    target_folder.pop();
    target_folder.push("temp/temp.png");
    // ve dosya adı kaynak dosya adı olarak değiştiriliyor
    target_folder.set_file_name(target_file_name?.as_str());

    // time ile dönüştürme işleminin ne kadar sürdüğünü ölçümlemeyi hedefliyoruz.
    let timer = Instant::now();
    // parametre olarak gelen bilgiyi kullanıp dosyası image modülünde yararlanarak açıyoruz.
    let img = image::open(&source_folder)?;
    // thumbnail işimizi epey kolaylaştırıyor. gelen genişlik ve yüksekliğe göre ölçeklenmiş yeni
    // resim oluşturuluyor
    let scaled = img.thumbnail(width, height);
    // çıktıyı hedef klasöre yazmak için bir dosya oluşturuyor
    let mut output = fs::File::create(&target_folder)?;
    // ve DynamicImage türünden olan scaled içeriğini buraya Png formatında yazdırıyoruz.
    scaled.write_to(&mut output, ImageFormat::Png)?;
    // süreyi ölçüyoruz ve ekrana bir bilgi basıyoruz.
    let elapsed_time = timer.elapsed();
    println!(
        "{:?},{}X{} boyutlarında {:?} sürede oluşturuldu",
        output, width, height, elapsed_time
    );
    Ok(())
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

    #[test]
    fn resize_image_works_test() {
        let mut source_directory = PathBuf::from("./images/bisiklet.jpg");
        let result = resize_image(250, 250, &mut source_directory);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn resize_image_fails_test() {
        let mut source_directory = PathBuf::from("./images/no_file.jpg");
        let result = resize_image(250, 250, &mut source_directory);
        assert_eq!(
            result,
            Err(ImagixError::ImageResizing(
                "Resim dönüştürme işlemi sırasında hata".to_string()
            ))
        );
    }

    #[test]
    fn resize_single_mode_test() {
        let mut source = PathBuf::from("./images/bisiklet.jpg");
        let process_result = process_resize_request(Size::Small, Mode::Single, &mut source);
        match process_result {
            Ok(_) => println!("Yeniden boyutlandırma işlemi başarılı."),
            Err(e) => println!("Hata {:?}", e),
        }
        assert_eq!(true, PathBuf::from("./images/temp/bisiklet.png").exists());
    }

    #[test]
    fn resize_full_mode_test() {
        let mut source = PathBuf::from("./images/");
        let process_result = process_resize_request(Size::Small, Mode::Full, &mut source);
        match process_result {
            Ok(_) => println!("Toplu yeniden boyutlandırma işlemi başarılı."),
            Err(e) => println!("Hata {:?}", e),
        }
        assert_eq!(true, PathBuf::from("./images/temp/bisiklet.png").exists());
        assert_eq!(true, PathBuf::from("./images/temp/horten.png").exists());
        assert_eq!(true, PathBuf::from("./images/temp/obiwanship.png").exists());
    }
}
