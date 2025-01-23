use clap::*;
use colored::Colorize;
use std::thread;
use std::time::Duration;
use sysinfo::{Process, System};

fn main() {
    let cli = Cli::parse();
    let system = prepare_sys();

    let mut processes: Vec<_> = system.processes().values().collect();

    if !cli.all {
        if let Some(ref filter_name) = cli.filter {
            processes.retain(|p| p.name().to_str().unwrap() == filter_name);
        }
    }

    if let Some(sort_field) = cli.sort {
        match sort_field {
            SortField::Cpu => {
                if let Some(order_field) = cli.order {
                    match order_field {
                        Order::Asc => processes
                            .sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap()),
                        Order::Desc => processes
                            .sort_by(|a, b| a.cpu_usage().partial_cmp(&b.cpu_usage()).unwrap()),
                    }
                } else {
                    processes
                        .sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap())
                }
            }
            SortField::Memory => {
                if let Some(order_field) = cli.order {
                    match order_field {
                        Order::Asc => processes.sort_by(|a, b| b.memory().cmp(&a.memory())),
                        Order::Desc => processes.sort_by(|a, b| a.memory().cmp(&b.memory())),
                    }
                } else {
                    processes.sort_by(|a, b| b.memory().cmp(&a.memory()))
                }
            }
        }
    }

    view_dashboard(&mut processes);
}

fn prepare_sys() -> System {
    let mut system = System::new_all();
    system.refresh_all();
    println!("{}", "Calculating... Just a second please...".yellow());
    thread::sleep(Duration::from_secs(2));
    system.refresh_all();
    system
}

fn view_dashboard(processes: &mut Vec<&Process>) {
    println!(
        "{:<10} {:<30} {:<10} {:<15} {:<15} {:<15}",
        "PID".bold().white(),
        "Process Name".bold().green(),
        "CPU %".bold().yellow(),
        "Memory".bold().red(),
        "Memory (MB)".bold().cyan(),
        "Memory (GB)".bold().bright_cyan()
    );

    for process in processes {
        let memory_mb = process.memory() as f64 / 1024.0;
        let memory_gb = memory_mb / (1024.0 * 1024.0);
        println!(
            "{:<10} {:<30} {:<10.4} {:<15} {:<15} {:<10.6}",
            process.pid().to_string().white(),
            process.name().to_os_string().into_string().unwrap().green(),
            process.cpu_usage().to_string().yellow(),
            process.memory().to_string().red(),
            memory_mb.to_string().cyan(),
            memory_gb.to_string().bright_cyan()
        );
    }
}

#[derive(Parser)]
#[command(name = "A Simple Process Monitor")]
#[command(version = "1.0")]
#[command(about = "Displays information about system processes")]
#[command(author = "Burak Selim Şenyurt")]
struct Cli {
    /// List all processes
    #[arg(long)]
    all: bool,

    /// Filter processes by name
    #[arg(short, long)]
    filter: Option<String>,

    /// Sort processes by 'cpu' or 'memory'
    #[arg(short, long, value_name = "cpu")]
    sort: Option<SortField>,

    /// Order processes by 'ascending' or 'descending'
    #[arg(short, long, value_name = "asc")]
    order: Option<Order>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SortField {
    Cpu,
    Memory,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Order {
    Asc,
    Desc,
}
