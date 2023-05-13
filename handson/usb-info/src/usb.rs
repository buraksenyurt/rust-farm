use std::fmt::{Display, Formatter};

pub struct Usb {
    manufacturer: String,
    product: String,
    address: u8,
    vendor_id: u16,
    product_id: u16,
    bus_number: u8,
    serial_num: String,
}

impl Usb {
    pub fn new(
        manufacturer: String,
        product: String,
        address: u8,
        vendor_id: u16,
        product_id: u16,
        bus_number: u8,
        serial_num: String,
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

impl Display for Usb {
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
