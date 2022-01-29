#[cfg(test)]
mod tests {
    use crate::game::{append_infos, read_infos, read_infos_to, write_info, Info};
    use std::fs::File;

    #[test]
    fn write_to_file_succeeded_test() {
        let prince_of_persia = Info::new(String::from("Prince of Persia"), 1, 20987);
        let barbarian = Info::new(String::from("Barbarian"), 2, 11905);
        assert_eq!(write_info(prince_of_persia), true);
        assert_eq!(write_info(barbarian), true);
    }

    #[test]
    #[should_panic] // Bu test içinde bir panic oluşacağını beklemekteyiz.
    fn write_to_file_error_test() {
        // Geçersiz bir dosya adı ile create operasyonunu çalıştırdığımızda
        // bir panik oluşacaktır. Bunu expect ile beklediğimizi de ifade ettik.
        File::create("~./*+.bin").expect("Dosya yazma hatası");
    }

    #[test]
    fn append_to_file_test() {
        let games = vec![
            Info::new(String::from("Dunkey Kong"), 1, 10001),
            Info::new(String::from("Pacman"), 2, 20451),
            Info::new(String::from("Super Mario"), 3, 12850),
            Info::new(String::from("Sensible Soccer"), 4, 24501),
            Info::new(String::from("Warcraft II"), 5, 39450),
        ];
        let result = append_infos(&games.as_slice());
        assert_eq!(result, true);
    }

    #[test]
    fn read_from_file_test() {
        let content = read_infos(String::from("src/games.data")).unwrap();
        assert!(content.len() > 1);
    }

    #[test]
    #[should_panic]
    fn read_from_file_error_test() {
        let _content = read_infos(String::from("src/nofile.data")).expect("Dosya bulunamadı");
        //assert_eq!(_content,None);
    }

    #[test]
    fn read_from_file_line_by_line_test() {
        let lines = read_infos_to();
        assert!(lines.len() > 0);
        assert_eq!(lines[0], "No:1|Dunkey Kong|10001 kez beğenildi.");
    }
}

/*
   Dosya yazma ve okuma işlemleri için built-in io modülünün enstrümanları kullanılır.
   Senaryoda Info isimli örnek bir struct var. Bu yapıya ait nesne içeriklerini
   oyunun sözde dat uzantılı veri dosyasına yazdırıyoruz.
*/
#[allow(dead_code)]
mod game {
    use std::fmt::{Display, Formatter};
    use std::fs::{File, OpenOptions};
    use std::io::{BufRead, BufReader, Read, Write};

    // Kobay veri yapımız
    pub struct Info {
        title: String,
        id: u32,
        like: u16,
    }

    impl Info {
        // Info nesnelerini new ile kolayca oluşturalım diye.
        pub fn new(title: String, id: u32, like: u16) -> Self {
            Info { title, id, like }
        }
    }

    // to_string için Display trait'ini implemente ettik
    impl Display for Info {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(
                f,
                "No:{}|{}|{} kez beğenildi.",
                &self.id, &self.title, &self.like
            )
        }
    }

    // Oyun için bir dosya oluşturup içerisinde veri yazan fonksiyonumuz.
    // Dosya create modda açıldığı için her seferinde sıfırdan oluşturulur.
    pub fn write_info(info: Info) -> bool {
        // create ile bir dosya oluşturmayo deniyoruz aslında.
        // sonucu match ile kontrol etmekteyiz. Error Handling konusunda expect de kullanabiliriz
        let gf = File::create(format!("src/game_{}.dat", info.id));
        match gf {
            Ok(mut f) => {
                // dosya başarılı bir şekilde oluştuysa kod bu dala geçer.
                // write_all işlemini de deniyoruz. İşlem sonucu yine Result<T,Err> olduğundan
                // match ile bir kontrol yapmaktayız. Ancak bu kez match macrosunu kullandık.
                // Nitekim w başarılı ise true değilse false dönüyoruz.
                let w = f.write_all(info.to_string().as_bytes());
                matches!(w, Ok(_))
            }
            _ => false,
        }
    }

    // Bu fonksiyon gelen info türünden vector'deki içerikleri games.data dosyasına ilave eder.
    // Dosyayı append modda açtığımız için sürekli olarak içerik arka arkaya eklenecektir.
    pub fn append_infos(infos: &[Info]) -> bool {
        // Öncelikle games/data dosyasını append modda açıyoruz.
        // open Result döndüğünden hata durumunu match ile kontrol altına alıyoruz.
        let gf = OpenOptions::new().append(true).open("src/games.data");
        match gf {
            Ok(mut f) => {
                // Dosya başarılı şekilde açıldıysa info nesnelerini dolaşıp dosyaya ekliyoruz.
                for info in infos {
                    let wr = f.write(info.to_string().as_bytes());
                    match wr {
                        Ok(_) => continue,
                        Err(_) => return false,
                    }
                }
                true
            }
            _ => false,
        }
    }

    // Dosya içeriğini komple okuyan fonksiyon.
    pub fn read_infos(file: String) -> Option<String> {
        // games.data içeriğinin tamamını bir String'e alacağız.
        let mut content = String::new();
        // Dosyayı açmayı deniyoruz.
        let gf = File::open(file);
        match gf {
            Ok(mut f) => {
                // Dosya açılmışsa kod bu dala akar.
                // Şimdi içeriğini okumaya çalışıyoruz.
                let r = f.read_to_string(&mut content);
                match r {
                    // İçerik başarılı okunduysa içeriğini dönmekteyiz
                    Ok(_) => Some(content),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    // Bu sefer games.data içeriğini satır satır okuyup bir String vector'e alıyoruz.
    pub fn read_infos_to() -> Vec<String> {
        let mut infos = Vec::<String>::new();
        // Dosyanın var olduğunu düşüüyoruz. Hatayı göz ardı ettik kısacası.(unwrap)
        let gf = File::open(String::from("src/games.data")).unwrap();
        // dosyayı kullanacak bir reader oluşturduk.
        let reader = BufReader::new(gf);
        // BufReader yardımıyla dosya içeriğini satır satır okuma şansımız var.
        for (_, line) in reader.lines().enumerate() {
            // Satırı okurken oluşabilecek hataları da göz ardı ettik.(unwrap)
            infos.push(line.unwrap().to_string());
        }
        infos
    }
}
