#[cfg(test)]
mod tests {
    use crate::game::{append_infos, write_info, Info};
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
    use std::io::Write;

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
            write!(
                f,
                "No:{}|{}|{} kez beğenildi.\n",
                &self.id, &self.title, &self.like
            )
        }
    }

    // Oyun için bir dosya oluşturup içerisinde veri yazan fonksiyonumuz
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
}
