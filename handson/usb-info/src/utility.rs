use crate::usb::Usb;
use crate::usb_error::UsbError;
use crate::usb_list::UsbList;
use libusb::{Context, Device, DeviceHandle};
use std::time::Duration;

pub struct Utility;

impl Utility {
    pub fn read() -> Result<UsbList, UsbError> {
        let context = Context::new()?;
        let mut usb_list = UsbList::new();
        for device in context.devices()?.iter() {
            let descriptor = device.device_descriptor()?;
            println!(
                "Vendor: {}, Product Id: {}",
                descriptor.vendor_id(),
                descriptor.product_id()
            );

            let handle =
                context.open_device_with_vid_pid(descriptor.vendor_id(), descriptor.product_id());
            match handle {
                Some(h) => {
                    println!("\n...getting usb info");
                    let usb_info = Self::get_device_info(device, h)?;
                    usb_list.add(usb_info);
                }
                None => {println!(
                    "Unsuccess for Vendor: {}, Product Id: {}",
                    descriptor.vendor_id(),
                    descriptor.product_id()
                );}
            }
        }
        Ok(usb_list)
    }

    fn get_device_info(device: Device, handle: DeviceHandle) -> Result<Usb, UsbError> {
        let descriptor = device.device_descriptor()?;
        let timeout = Duration::from_secs(1);
        let languages = handle.read_languages(timeout)?;
        let lang = languages[0];
        let manufacturer = handle.read_manufacturer_string(lang, &descriptor, timeout)?;
        let product = handle.read_product_string(lang, &descriptor, timeout)?;
        let read_serial = handle.read_serial_number_string(lang, &descriptor, timeout);
        let serial_number = match read_serial {
            Ok(s) => s,
            Err(_) => "Not Available".to_string(),
        };
        let usb = Usb::new(
            manufacturer,
            product,
            device.address(),
            descriptor.vendor_id(),
            descriptor.product_id(),
            device.bus_number(),
            serial_number,
        );
        Ok(usb)
    }
}
