use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Action {
    Get,
    Create,
    Delete,
    Edit,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "create" => Ok(Action::Create),
            "get" => Ok(Action::Get),
            "delete" => Ok(Action::Delete),
            "edit" => Ok(Action::Edit),
            _ => Err("Komut bulunamadı".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::action::Action;
    use std::str::FromStr;

    #[test]
    fn should_parse_valid_actions() {
        assert_eq!(Action::from_str("create").unwrap(), Action::Create);
        assert_eq!(Action::from_str("edit").unwrap(), Action::Edit);
        assert_eq!(Action::from_str("delete").unwrap(), Action::Delete);
        assert_eq!(Action::from_str("get").unwrap(), Action::Get);
    }

    #[test]
    #[should_panic]
    fn should_invalid_action_panics() {
        assert_eq!(Action::from_str("ping").unwrap(), Action::Create);
    }
}
