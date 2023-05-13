use crate::usb_error::UsbError;
use crate::utility::Utility;

mod test;
mod usb;
mod usb_error;
mod usb_list;
mod utility;

fn main() -> Result<(), UsbError> {
    println!("Reading USB informations...");
    let data = Utility::read()?;
    data.list();
    data.write_to_file()?;
    println!("Informations have been write to file.");
    Ok(())
}
