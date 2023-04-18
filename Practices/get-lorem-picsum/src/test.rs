#[cfg(test)]
mod test {
    use crate::builder::{get_photos};
    use crate::photo::Photo;

    #[test]
    pub fn should_get_file_name_works() {
        let photo = Photo {
            id: "1".to_string(),
            author: "burak selim senyurt".to_string(),
            width: 1080,
            height: 920,
            url: String::new(),
            download_url: String::new(),
        };
        let actual = photo.create_file_name();
        let expected = "burak_selim_senyurt_1080_920_1.jpg".to_string();
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    pub async fn should_overlimit_range_fail_on_get_photos() {
        let actual = get_photos(1, 50).await;
        assert!(actual.is_err());
    }
}
