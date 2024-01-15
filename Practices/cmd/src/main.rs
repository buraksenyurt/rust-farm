use std::fmt::{Display, Formatter};
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = CommandParser::parse(&args);
    command.execute(args);
}

trait Command {
    fn execute(&self, args: Vec<String>);
}

struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() == 5 {
            let title = args[2].to_string();
            let price = f32::from_str(args[3].as_str()).unwrap_or_default();
            let in_stock = bool::from_str(args[4].as_str()).unwrap_or_default();
            println!("--add işletilecek. Ürün adı {title} , Fiyat {price} , Stokta mı? {in_stock}");
        }
    }
}

struct ListCommand;

impl Command for ListCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() == 4 {
            let field_name = args[2].to_string();
            let ordering = Ordering::from_str(args[3].as_str()).unwrap_or_default();
            println!("--list işletilecek. Parametreler -> {field_name} , {ordering}");
        }
    }
}

struct FindCommand;

impl Command for FindCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() == 5 {
            let field_name = args[2].to_string();
            let cmp_operator = CompareOperator::from_str(args[3].as_str()).unwrap_or_default();
            let value = args[4].to_string();
            println!("--find işletilecek. {field_name} {cmp_operator} {value}");
        }
    }
}

struct UnknownCommand;

impl Command for UnknownCommand {
    fn execute(&self, _args: Vec<String>) {
        println!("Geçersiz komut");
    }
}

struct CommandParser;

impl CommandParser {
    pub fn parse(args: &[String]) -> Box<dyn Command> {
        match args.get(1).map(String::as_str) {
            Some("--add") => Box::new(AddCommand),
            Some("--list") => Box::new(ListCommand),
            Some("--find") => Box::new(FindCommand),
            _ => Box::new(UnknownCommand),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
enum Ordering {
    #[default]
    Ascending,
    Descending,
}
impl FromStr for Ordering {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "desc" => Ok(Self::Descending),
            _ => Ok(Self::Ascending),
        }
    }
}

impl Display for Ordering {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ascending => write!(f, "ascending"),
            Self::Descending => write!(f, "descending"),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
enum CompareOperator {
    #[default]
    Equal,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    NotEqual,
}

impl FromStr for CompareOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "eq" => Self::Equal,
            "neq" => Self::NotEqual,
            "gt" => Self::GreaterThan,
            "gte" => Self::GreaterThanEqual,
            "lt" => Self::LessThan,
            "lte" => Self::LessThanEqual,
            _ => Self::Equal,
        })
    }
}

impl Display for CompareOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompareOperator::Equal => {
                write!(f, "=")
            }
            CompareOperator::GreaterThan => {
                write!(f, ">")
            }
            CompareOperator::GreaterThanEqual => {
                write!(f, ">=")
            }
            CompareOperator::LessThan => {
                write!(f, "<")
            }
            CompareOperator::LessThanEqual => {
                write!(f, "<=")
            }
            CompareOperator::NotEqual => {
                write!(f, "!=")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{CompareOperator, Ordering};
    use std::str::FromStr;

    #[test]
    fn from_str_to_ordering_test() {
        let actual = Ordering::from_str("desc").unwrap();
        let expected = Ordering::Descending;
        assert_eq!(actual, expected);
        let actual = Ordering::from_str("middle").unwrap();
        let expected = Ordering::Ascending;
        assert_eq!(actual, expected);
    }

    #[test]
    fn from_str_to_compare_operator_test() {
        let test_cases = vec![
            ("eq", CompareOperator::Equal),
            ("neq", CompareOperator::NotEqual),
            ("gt", CompareOperator::GreaterThan),
            ("gte", CompareOperator::GreaterThanEqual),
            ("lt", CompareOperator::LessThan),
            ("lte", CompareOperator::LessThanEqual),
        ];
        for (input, expected) in test_cases {
            let actual = CompareOperator::from_str(input).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
