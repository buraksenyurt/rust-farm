use colored::Colorize;
use sysinfo::Process;

pub fn view_dashboard(processes: &[&Process]) {
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
            "{:<10} {:<30} {:<10.4} {:<15} {:<15} {:<15.6}",
            process.pid().to_string().white(),
            process
                .name()
                .to_os_string()
                .into_string()
                .unwrap_or_else(|_| "Unknown".to_string())
                .green(),
            process.cpu_usage().to_string().yellow(),
            process.memory().to_string().red(),
            memory_mb.to_string().cyan(),
            memory_gb.to_string().bright_cyan()
        );
    }
}
