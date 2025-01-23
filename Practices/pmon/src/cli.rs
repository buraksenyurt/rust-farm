use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "A Simple Process Monitor")]
#[command(version = "1.0")]
#[command(about = "Displays information about system processes")]
#[command(author = "Burak Selim Şenyurt")]
pub struct Cli {
    /// List all processes
    #[arg(long)]
    pub all: bool,

    /// Filter processes by name
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Sort processes by 'cpu' or 'memory'
    #[arg(short, long, value_name = "cpu")]
    pub sort: Option<SortField>,

    /// Order processes by 'ascending' or 'descending'
    #[arg(short, long, value_name = "asc")]
    pub order: Option<Order>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SortField {
    Cpu,
    Memory,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Order {
    Asc,
    Desc,
}
