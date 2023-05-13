#[cfg(test)]
mod test {
    use crate::usb::Usb;
    use crate::usb_list::UsbList;

    #[test]
    fn should_write_to_file_works() {
        let usb_1 = Usb::new("kingston", "16 Gb", 15, 81, 120, 3, "USB-KNG-1023");
        let usb_2 = Usb::new("western digital", "128 Gb", 16, 181, 98, 5, "USB-WD-5654");
        let mut list = UsbList::new();
        list.add(usb_1);
        list.add(usb_2);
        let result = &list.write_to_file();
        assert!(result.is_ok());
    }
}
