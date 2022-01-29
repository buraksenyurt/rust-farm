#[cfg(test)]
mod tests {
    use crate::game::{create_file, Info};

    #[test]
    fn write_to_file_succeeded_test() {
        let prince_of_persia = Info::new(String::from("Prince of Persia"), 1, 20987);
        let barbarian = Info::new(String::from("Barbarian"), 2, 11905);
        assert_eq!(create_file(prince_of_persia), true);
        assert_eq!(create_file(barbarian), true);
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
    use std::fs::File;
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
                "No:{}|{}|{} kez beğenildi.",
                &self.id, &self.title, &self.like
            )
        }
    }

    // Oyun için bir dosya oluşturup içerisinde veri yazan fonksiyonumuz
    pub fn create_file(info: Info) -> bool {
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
}
