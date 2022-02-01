use crossbeam::channel::{Receiver, Sender};
use log::{error, info};
use std::thread;
use std::time::Duration;

// Yaptıracağımız işleri tutan bir enum türü. Receiver tarafından kullanılır.
#[derive(Debug, Copy, Clone)]
pub enum Job {
    WheatFarm,
    FishFarm,
    Shack(u8),       // Kaç kişilik bir kulübe olacağını u8 ile atabiliriz
    ArcherTower(u8), // Belki u8 ile okçu kulesinin seviyesini ifade ederiz
    Ditch(f32),      // hendeğin uzunluğunu u32 ile alabiliriz
}

// İşler tamamlandıktan sonra kanala bırakacağımız mesajlar için aşağıdaki enum kullanılabilir.
// Sender tarafından kullanılır.
#[derive(Debug, Copy, Clone)]
pub enum Harvest {
    WheatFarm,
    FishFarm,
    Shack,
    ArcherTower,
    Ditch,
}

// Fonksiyon Receiver ve Sender türünden iki parametre almakta.
// Buna göre kanaldan mesaj alma ve kanala mesaj bırakma işlevlerini üstlendiğini ifade edebiliriz.
pub fn pesant_worker(jobs: Receiver<Job>, results: Sender<Harvest>) {
    // Bir döngü ile gelen Job listesini dolaşıyoruz.
    for job in jobs {
        // her bir Job'u match ifadesi ile kontrol ediyor ve sembolik bir gecikme ile işletip
        // Sender için bir sonuç alıyoruz.
        let response = match job {
            Job::ArcherTower(l) => {
                info!("{} seviyesinde okçu kulesi inşaası", l);
                thread::sleep(Duration::from_secs_f32(2.0));
                Harvest::ArcherTower
            }
            Job::Ditch(l) => {
                info!("{} uzunluğunda hendek.", l);
                thread::sleep(Duration::from_secs_f32(1.5));
                Harvest::Ditch
            }
            Job::FishFarm => {
                info!("Kıyıya balık çifliği inşaası.");
                thread::sleep(Duration::from_secs_f32(3.5));
                Harvest::FishFarm
            }
            Job::WheatFarm => {
                info!("Buğday tarlası inşaası.");
                thread::sleep(Duration::from_secs_f32(0.5));
                Harvest::WheatFarm
            }
            Job::Shack(p) => {
                info!("{} kişilik kulübe inşaası.", p);
                thread::sleep(Duration::from_secs_f32(2.75));
                Harvest::Shack
            }
        };
        info!("Yapılan iş {:?}", response);
        // İstenen işlem tamamlandıktan sonra sonucu Sender ile kanala bırakmaktayız.
        // send işlemi sırasında bir hata olma ihtimaline karşı da durumu kontrol ediyoruz.
        if results.send(response).is_err() {
            error!("Ups bir hata oluştu.");
            break;
        }
    }
}
