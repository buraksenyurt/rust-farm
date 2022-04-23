mod imager;
mod imagix;

#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn read_path_content_test() {
        let entries = fs::read_dir("./").unwrap();
        for entry in entries {
            if let Ok(entry) = entry {
                assert_eq!(entry.path().exists(), true);
            }
        }
    }
}
