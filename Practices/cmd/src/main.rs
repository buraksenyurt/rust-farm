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
            println!("--add işletilecek. Parametreler -> {title} , {price} , {in_stock}");
        }
    }
}

struct ListCommand;

impl Command for ListCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() == 4 {
            let args_1 = args[2].to_string();
            let args_2 = args[3].to_string();
            println!("--list işletilecek. Parametreler -> {args_1} , {args_2}");
        }
    }
}

struct FindCommand;

impl Command for FindCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() == 3 {
            let args_1 = args[2].to_string();
            println!("--find işletilecek. Parametreler -> {args_1}");
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
