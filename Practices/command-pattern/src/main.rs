use std::fmt::{Display, Formatter};

/*
   Senaryoda bir migration işlemi ele alınıyor.
   Tablo oluşturma ve kolon ekleme gibi işleri belli bir sıra ile yapmak istiyoruz.
   Command tasarım kalıbını bu senaryoya uyarlamaktayız.
*/
fn main() {
    let mut init = Plan::new();
    let table = Table {
        name: "Product".to_string(),
    };
    let cmd = Box::new(table);
    init.add_command(cmd);

    let cmd = Box::new(Column {
        table: "Product".to_string(),
        data_type: DataType::Text(10),
        allow_null: false,
        name: "title".to_string(),
    });
    init.add_command(cmd);

    let cmd = Box::new(Column {
        table: "Product".to_string(),
        data_type: DataType::Decimal(8),
        allow_null: false,
        name: "unit_price".to_string(),
    });
    init.add_command(cmd);

    let cmd = Box::new(Column {
        table: "Product".to_string(),
        data_type: DataType::Number(4),
        allow_null: false,
        name: "stock_size".to_string(),
    });
    init.add_command(cmd);

    let cmd = Box::new(Column {
        table: "Product".to_string(),
        data_type: DataType::Bool,
        allow_null: false,
        name: "on_sale".to_string(),
    });
    init.add_command(cmd);

    init.execute();
    init.rollback();
}

enum ProcessResult {
    Completed,
    Error,
}
trait Migration {
    fn up(&self) -> ProcessResult;
    fn down(&self) -> ProcessResult;
}

struct Table {
    pub name: String,
}
impl Migration for Table {
    fn up(&self) -> ProcessResult {
        println!("CREATE TABLE {}", self.name);
        ProcessResult::Completed
    }

    fn down(&self) -> ProcessResult {
        println!("DROP TABLE {}", self.name);
        ProcessResult::Completed
    }
}

enum DataType {
    Text(u32),
    Number(u32),
    Decimal(u32),
    Bool,
}
impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Text(size) => {
                write!(f, "TEXT ({})", size)
            }
            DataType::Number(size) => {
                write!(f, "NUMBER ({})", size)
            }
            DataType::Decimal(size) => {
                write!(f, "DECIMAL ({})", size)
            }
            DataType::Bool => {
                write!(f, "BOOL")
            }
        }
    }
}
struct Column {
    pub table: String,
    pub name: String,
    pub data_type: DataType,
    pub allow_null: bool,
}

impl Migration for Column {
    fn up(&self) -> ProcessResult {
        println!(
            "ALTER TABLE {} ADD COLUMN {} {} {}",
            self.table,
            self.name,
            self.data_type,
            if self.allow_null {
                "NULLABLE"
            } else {
                "NOT NULLABLE"
            }
        );
        ProcessResult::Completed
    }

    fn down(&self) -> ProcessResult {
        println!("ALTER TABLE {} DROP COLUMN {}", self.table, self.name);
        ProcessResult::Completed
    }
}

struct Plan {
    commands: Vec<Box<dyn Migration>>,
}

impl Plan {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_command(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<ProcessResult> {
        self.commands.iter().map(|cmd| cmd.up()).collect()
    }

    fn rollback(&self) -> Vec<ProcessResult> {
        self.commands.iter().rev().map(|cmd| cmd.down()).collect()
    }
}
