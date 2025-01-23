mod cli;
mod dashboard;
mod process_manager;

use crate::cli::Cli;
use crate::dashboard::view_dashboard;
use crate::process_manager::*;
use clap::*;

fn main() {
    let cli = Cli::parse();
    let system = prepare_sys();
    let mut processes: Vec<_> = system.processes().values().collect();

    apply_filters(&cli, &mut processes);
    sort_processes(&cli, &mut processes);

    view_dashboard(&processes);
}
