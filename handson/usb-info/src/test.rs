#[cfg(test)]
mod test {
    use crate::usb::Usb;
    use crate::usb_list::UsbList;

    #[test]
    fn should_write_to_file_works() {
        let usb_1 = Usb::new(
            "kingston".to_string(),
            "16 Gb".to_string(),
            15,
            81,
            120,
            3,
            "USB-KNG-1023".to_string(),
        );
        let usb_2 = Usb::new(
            "western digital".to_string(),
            "128 Gb".to_string(),
            16,
            181,
            98,
            5,
            "USB-WD-5654".to_string(),
        );
        let mut list = UsbList::new();
        list.add(usb_1);
        list.add(usb_2);
        let result = &list.write_to_file();
        assert!(result.is_ok());
    }
}
