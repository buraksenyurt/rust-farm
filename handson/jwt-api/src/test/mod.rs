#[cfg(test)]
pub mod test {
    use crate::data::security::{create_hashed_pwd, verify_pwd};

    #[test]
    fn verify_password_test() {
        let hashed_value = create_hashed_pwd("P@ssw0rd");
        let actual = verify_pwd("P@ssw0rd", &hashed_value);
        let expected=true;
        assert_eq!(actual, expected);
    }
}
