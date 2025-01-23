use crate::cli::*;
use colored::Colorize;
use std::thread;
use std::time::Duration;
use sysinfo::*;

pub fn prepare_sys() -> System {
    let mut system = System::new_all();
    system.refresh_all();
    println!("{}", "Calculating... Just a second please...".yellow());
    thread::sleep(Duration::from_secs(2));
    system.refresh_all();
    system
}

pub fn apply_filters(cli: &Cli, processes: &mut Vec<&Process>) {
    if !cli.all {
        if let Some(ref filter_name) = cli.filter {
            processes.retain(|p| p.name().to_str().unwrap_or("") == filter_name);
        }
    }
}

pub fn sort_processes(cli: &Cli, processes: &mut Vec<&Process>) {
    if let Some(sort_field) = cli.sort {
        let order = cli.order.unwrap_or(Order::Desc);

        match (sort_field, order) {
            (SortField::Cpu, Order::Asc) => {
                processes.sort_by(|a, b| a.cpu_usage().partial_cmp(&b.cpu_usage()).unwrap());
            }
            (SortField::Cpu, Order::Desc) => {
                processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
            }
            (SortField::Memory, Order::Asc) => {
                processes.sort_by(|a, b| a.memory().cmp(&b.memory()));
            }
            (SortField::Memory, Order::Desc) => {
                processes.sort_by(|a, b| b.memory().cmp(&a.memory()));
            }
        }
    }
}
