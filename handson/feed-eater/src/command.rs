use crate::feed::Feed;

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(Feed),
    Delete,
    GetFeeds,
    SelectAll,
    Top(Option<u8>),
    Undefined,
}
pub fn get_command(args: Vec<String>) -> Command {
    if args.is_empty() {
        return Command::Undefined;
    }
    if args.len() == 4 && args[1].to_uppercase() == "ADD" {
        return Command::Add(Feed::new(args[2].to_string(), args[3].to_string()));
    }
    if args.len() == 3 {
        if args[1].to_uppercase() == "TOP" {
            if let Ok(value) = args[2].parse::<u8>() {
                return Command::Top(Some(value));
            } else {
                return Command::Top(Some(5));
            }
        }
    } else if args.len() == 2 {
        if args[1].to_uppercase() == "ALL" {
            return Command::SelectAll;
        } else if args[1].to_uppercase() == "FEEDS" {
            return Command::GetFeeds;
        } else if args[1].to_uppercase() == "DEL" {
            return Command::Delete;
        }
    }
    Command::Undefined
}

#[cfg(test)]
mod tests {
    use crate::command::{get_command, Command};
    use crate::feed::Feed;

    #[test]
    fn top_8_expression_can_convert_to_command_test() {
        let input = "-- top 8";
        let commands = input.split_whitespace().map(str::to_string).collect();
        let command = get_command(commands);
        assert_eq!(command, Command::Top(Some(8)));
    }
    #[test]
    fn top_expression_can_convert_to_default_command_test() {
        let input = "-- top eigth";
        let commands = input.split_whitespace().map(str::to_string).collect();
        let command = get_command(commands);
        assert_eq!(command, Command::Top(Some(5)));
    }
    #[test]
    fn all_can_convert_to_select_all_command_test() {
        let input = "-- aLL";
        let commands = input.split_whitespace().map(str::to_string).collect();
        let command = get_command(commands);
        assert_eq!(command, Command::SelectAll);
    }
    #[test]
    fn feeds_can_convert_to_get_feeds_command_test() {
        let input = "-- FeEDs";
        let commands = input.split_whitespace().map(str::to_string).collect();
        let command = get_command(commands);
        assert_eq!(command, Command::GetFeeds);
    }
    #[test]
    fn unknowns_can_convert_to_undefined_command_test() {
        let input = "bla bla bla 3.14";
        let commands = input.split_whitespace().map(str::to_string).collect();
        let command = get_command(commands);
        assert_eq!(command, Command::Undefined);
    }
    #[test]
    fn empty_expressions_can_convert_to_undefined_command_test() {
        let input = "";
        let commands = input.split_whitespace().map(str::to_string).collect();
        let command = get_command(commands);
        assert_eq!(command, Command::Undefined);
    }

    #[test]
    fn add_feed_expressions_can_convert_to_command_test() {
        let input = "-- add test http://feedly/feed";
        let commands = input.split_whitespace().map(str::to_string).collect();
        let command = get_command(commands);
        let expected = Command::Add(Feed::new(
            String::from("test"),
            String::from("http://feedly/feed"),
        ));
        assert_eq!(command, expected);
    }

    #[test]
    fn del_feed_expressions_can_convert_to_command_test() {
        let input = "-- del";
        let commands = input.split_whitespace().map(str::to_string).collect();
        let command = get_command(commands);
        let expected = Command::Delete;
        assert_eq!(command, expected);
    }
}
