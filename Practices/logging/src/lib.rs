// log paketinden kullanacağımız macro'lar için gerekli bildirimler
use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Voyager {
    pub life: u8,
    pub nickname: String,
    pub universe: String,
    pub is_active: bool,
}

impl Voyager {
    pub fn new(nickname: String) -> Self {
        // debug türünden bir log bırakıyoruz
        debug!(target:"App Logs","Oyuna {} isimli bir gezgin katılıyor.",nickname);
        Voyager {
            nickname,
            ..Default::default()
        }
    }

    pub fn connect(&mut self, universe: String) {
        if !self.is_active && self.life > 0 {
            // info türünden bir log bırakıyoruz
            info!(target:"App Logs","{}, {} evrenine bağlanıyor",self.nickname,universe);
            self.is_active = true;
            self.universe = universe;
        }
    }

    pub fn hited(&mut self) {
        self.life -= 1;
        // warn türünden bir log bırakıyoruz
        warn!(target:"App Logs","{} vuruldu ve {} canı kaldı.",self.nickname,self.life);

        if self.life == 0 {
            // error türünden bir log bırakıyoruz
            error!(target:"App Logs","{} ne yazık ki tüm canlarını kaybetti. Bağlantısı kesiliyor",self.nickname);
            self.is_active = false;
        }
    }
}

impl Default for Voyager {
    fn default() -> Self {
        let voyager = Voyager {
            life: 3,
            is_active: false,
            universe: String::from("nowhere"),
            nickname: String::from("unknown"),
        };
        // trace türünden bir log bırakıyoruz
        trace!(target:"App Logs","Gezgin için varsayılan değerler yükleniyor.{:?}",voyager);
        voyager
    }
}
