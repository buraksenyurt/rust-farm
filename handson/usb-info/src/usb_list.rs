use crate::usb::Usb;
use crate::usb_error::UsbError;
use std::fs::File;
use std::io::Write;

pub struct UsbList {
    data: Vec<Usb>,
}

impl UsbList {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
    pub fn add(&mut self, usb: Usb) {
        self.data.push(usb);
    }
    pub fn list(&self) {
        for usb in self.data.iter() {
            println!("{}", usb);
        }
    }
    pub fn write_to_file(&self) -> Result<(), UsbError> {
        let mut f = File::create("usb_info.dat")?;
        for usb in self.data.iter() {
            write!(f, "{}\n", usb)?;
        }
        Ok(())
    }
}
