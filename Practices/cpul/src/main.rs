use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::thread::sleep;
use std::time::Duration;
use sysinfo::System;
use std::io::Write;

fn main() {
    let mut sys = System::new();
    let mut measurements = VecDeque::new();
    let max_log_count = sys.cpus().len() * 5;
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    loop {
        sys.refresh_cpu_usage();

        for cpu in sys.cpus() {
            // println!("{:?}", cpu);

            let level = match cpu.cpu_usage() {
                0.0..=25.0 => Level::EASY,
                26.0..=50.0 => Level::NORMAL,
                51.0..=80.0 => Level::HEATING,
                81.0..=100.0 => Level::CRITICAL,
                _ => Level::UNKNOWN,
            };

            if measurements.len() >= max_log_count {
                measurements.pop_front();
            }
            measurements.push_back(CpuLog {
                name: cpu.name().to_string(),
                level,
                time_stamp: Utc::now(),
                usage: cpu.cpu_usage(),
            });

            if let Err(e) = writeln!(lock, "{}", measurements.back().unwrap()) {
                eprintln!("Error writing measurements to stdout. {}", e);
            }
            // println!("{}", measurements.back().unwrap());
            // sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            sleep(Duration::from_secs(1));
        }
    }
}

struct CpuLog {
    name: String,
    level: Level,
    time_stamp: DateTime<Utc>,
    usage: f32,
}

enum Level {
    EASY,
    NORMAL,
    HEATING,
    CRITICAL,
    UNKNOWN,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::EASY => write!(f, "EASY"),
            Level::NORMAL => write!(f, "NORMAL"),
            Level::HEATING => write!(f, "HEATING"),
            Level::CRITICAL => write!(f, "CRITICAL"),
            Level::UNKNOWN => write!(f, "UNKNOWN"),
        }
    }
}

impl Display for CpuLog {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{}|{} (%)|{}",
            self.time_stamp, self.name, self.usage, self.level
        )
    }
}
