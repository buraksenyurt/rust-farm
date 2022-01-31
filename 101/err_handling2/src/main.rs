use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;

#[allow(dead_code)]
#[derive(Debug)]
enum ProcessError {
    Danger,
}
impl Error for ProcessError {}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Materyalde element ihlali")
    }
}

#[allow(dead_code)]
fn process(material: String) -> Result<bool, ProcessError> {
    if material.to_lowercase().contains("nitro") {
        Err(ProcessError::Danger)
    } else {
        Ok(true)
    }
}

/*
   Öncelikle process isimli fonksiyona odaklanalım. Gelen içerikte nitro kelimesi geçiyorsa,
   bir ProcessError nesnesini Error olarak döndürmekte. Aksi takdirde işlerin yolunda gittiğine
   karar verip true dönüyor.

   Bu fonksiyon Result<T,Err> döndüğünden çağırdığımız yerde kontrol atlına almak isteyeceğiz.
   İlk akla gelen yol aşağıdaki gibi pattern matching kullanmak olacaktır. Err bloğunda,
   dikkat edileceği üzere hata yakalanır ve yine de kasıtlı olarak panic oluşturlur.
*/

fn main() {
    // let result = process(String::from("3 Karbon 1 nitro gliserin 2 azot"));
    // match result {
    //     Ok(r) => println!("İşlem sonucu {}", r),
    //     Err(e) => {
    //         println!("{:?}", e.to_string());
    //         panic!("Dikkat nitro");
    //     }
    // }

    /*
        Yukarıdaki gibi Result dönüşlerinde Err kontrolü yapıp panic oluşmasını istiyorsak,
        işi biraz daha kısaltmak mümkün. unwrap ve expect fonksiyonlarını bu amaçla kullanabiliriz.
        unwrap'ta eğer işlem başarılı ise doğrudan sonuca ulaşırız. Hata varsa'da panic oluşturulur.

        unwrap yerine expect tercih edersek yine hata durumlarında panik oluşmasını sağlarız ama
        bu sefer ek bir bilgi mesajı da verebiliriz.
    */
    // let result = process(String::from("2 azot 1 karbon 3 oksijen")).unwrap();
    // println!("İşlem sonucu {:?}", result);
    //
    // let result = process(String::from("2 azot 1 nitro")).unwrap();
    // println!("İşlem sonucu {:?}", result);

    // let result = process(String::from("2 azot 1 karbon 3 oksijen")).expect("nitro var, kaçın!!!");
    // println!("İşlem sonucu {:?}", result);
    //
    // let result = process(String::from("2 azot 1 nitro")).expect("nitro var, kaçın!!!");
    // println!("İşlem sonucu {:?}", result);

    // Fonksiyon çağrıldığında serial.dat ortamda yoksa hata mesajı elde edilir
    // ama program panikleyip sonlanmaz.
    let serial_number = read_serial_num_from_file();
    println!("{:?}", serial_number);

    // Testlerde src altına serial.dat şeklinde bir dosya ekleyerek denemek lazım.

    let serial_number = read_serial_number();
    println!("{:?}", serial_number);

    println!("Program devam ediyor");
}

/*
   Hata yönetiminde işleri kolaylaştıran birde ? operatörü vardır.
   Bu durumu anlamak için aşağıdaki fonksiyonu ele alalım.
   Bu kobay fonksiyon deneysel olarak serial.dat dosyasından bir key okuma işlevini icra etmekte.

   Dosya açma ve içeriği okumak için kullanılan open ve read_to_string sonuçlarını match ifadeleri
   ile kontrol altına alıyor buna göre Result dönüşünü besliyoruz.

   Aşağıdaki fonksiyonu ? operatörünü kullanarak daha şık bir formatta yazabiliriz.
   bknz read_serial_number fonksiyonu
*/
fn read_serial_num_from_file() -> Result<String, io::Error> {
    // Dosyayı aç
    let f = File::open("src/serial.dat");
    // Operasyon sonucuna bak
    let mut f = match f {
        Ok(file) => file,        // dosya varsa f'e atar
        Err(e) => return Err(e), // Hata varsa geriye hata nesnesi döner. panic değil!!!
    };

    let mut serial = String::new();
    // içerik okunursa geriye döner, bulamassa yine Err nesnesi verir.
    match f.read_to_string(&mut serial) {
        Ok(_) => Ok(serial),
        Err(e) => Err(e),
    }
}

// Bu fonksiyonda yukarıdaki match ifadeleri yerini ? operatörü kullanımına bırakmıştır.
fn read_serial_number() -> Result<String, io::Error> {
    let mut f = File::open("src/serial.dat")?;
    let mut serial = String::new();
    f.read_to_string(&mut serial)?;
    Ok(serial)
}
