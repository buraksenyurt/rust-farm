use std::fmt::{Display, Formatter};

fn main() {
    println!("Hello, world!");
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
    Decimal(f32),
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
    pub table: Table,
    pub name: String,
    pub data_type: DataType,
    pub allow_null: bool,
}

impl Migration for Column {
    fn up(&self) -> ProcessResult {
        println!(
            "ALTER TABLE {} ADD COLUMN {} {} {}",
            self.table.name,
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
        println!("DROP TABLE {}", self.name);
        ProcessResult::Completed
    }
}
