#[cfg(test)]
mod test {
    use crate::argument::{Command, CommandError, List};
    use crate::photo::Photo;
    use std::str::FromStr;

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

    // #[tokio::test]
    // pub async fn should_overlimit_range_fail_on_get_photos() {
    //     let actual = get_photos(1, 50).await;
    //     assert!(actual.is_err());
    // }

    #[test]
    pub fn should_arg_many_can_convert_to_valid_command() {
        let argument = "mAny";
        let command = Command::from_str(argument);
        assert!(command.is_ok());
        assert_eq!(command.unwrap(), Command::Many(List::new(1, 10)));
    }

    #[test]
    pub fn should_arg_single_can_convert_to_valid_command() {
        let argument = "siNGLe";
        let command = Command::from_str(argument);
        assert!(command.is_ok());
        assert_eq!(command.unwrap(), Command::Single(1));
    }

    #[test]
    pub fn should_invalid_arg_can_convert_to_error() {
        let argument = "mAny off them";
        let command = Command::from_str(argument);
        assert!(command.is_err());
        assert_eq!(command.err().unwrap(), CommandError::Invalid);
    }
}
