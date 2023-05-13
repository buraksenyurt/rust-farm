use std::fmt::{Display, Formatter};

pub struct Usb<'a> {
    manufacturer: &'a str,
    product: &'a str,
    address: u8,
    vendor_id: u8,
    product_id: u8,
    bus_number: u8,
    serial_num: &'a str,
}

impl<'a> Usb<'a> {
    pub fn new(
        manufacturer: &'a str,
        product: &'a str,
        address: u8,
        vendor_id: u8,
        product_id: u8,
        bus_number: u8,
        serial_num: &'a str,
    ) -> Self {
        Self {
            manufacturer,
            product,
            address,
            vendor_id,
            product_id,
            bus_number,
            serial_num,
        }
    }
}

impl<'a> Display for Usb<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Manufacturer: {}\nProduct: {}\nAddress: {}\nVendor Id: {}\nProduct Id: {}\nBus Number: {}\nSerial Number: {}",
        self.manufacturer,
        self.product,
        self.address,
        self.vendor_id,
        self.product_id,
        self.bus_number,
        self.serial_num)
    }
}
