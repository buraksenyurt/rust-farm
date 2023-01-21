#[cfg(test)]
pub mod test {
    use crate::model::user::User;
    use crate::security::{create_hashed_pwd, create_jwt, verify_pwd};

    #[test]
    fn verify_password_test() {
        let hashed_value = create_hashed_pwd("P@ssw0rd");
        let actual = verify_pwd("P@ssw0rd", &hashed_value);
        let expected = true;
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn create_jwt_token_test() {
    //     let user = User {
    //         id: 1,
    //         username: "scoth".to_string(),
    //         password: "scoth@1234".to_string(),
    //         role: "Admin".to_string(),
    //     };
    //     let token = create_jwt(&user);
    //     assert!(token.len() > 10);
    // }
}
