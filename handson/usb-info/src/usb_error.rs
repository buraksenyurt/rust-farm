use libusb::Error;

pub struct UsbError {
    pub err: String,
}

impl From<Error> for UsbError {
    fn from(_value: Error) -> Self {
        Self {
            err: "USB Access problem".to_string(),
        }
    }
}

impl From<std::io::Error> for UsbError {
    fn from(value: std::io::Error) -> Self {
        Self {
            err: value.to_string(),
        }
    }
}
