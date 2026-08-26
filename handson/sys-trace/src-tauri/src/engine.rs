use crate::metrics::HostInfo;
use crate::metrics::Metrics;
use sysinfo::{Disks, System};

pub fn get_host_info(sys: &System) -> HostInfo {
    let name = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let os = System::long_os_version().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let architecture = std::env::consts::ARCH.to_string();
    let cpu_brand = sys
        .cpus()
        .first()
        .map_or("Unknown".to_string(), |cpu| cpu.brand().to_string());
    let physical_cores = System::physical_core_count();
    let logical_cores = sys.cpus().len();

    HostInfo {
        name,
        os,
        kernel_version,
        architecture,
        cpu_brand,
        physical_cores,
        logical_cores,
    }
}

pub fn collect(sys: &mut System, disks: &mut Disks) -> Metrics {
    sys.refresh_cpu_usage();
    sys.refresh_cpu_frequency();
    sys.refresh_memory();
    disks.refresh(true);

    let cpu_cores = sys
        .cpus()
        .iter()
        .map(|cpu| crate::metrics::CpuCore {
            name: cpu.name().to_string(),
            frequency: cpu.frequency(),
            usage: cpu.cpu_usage(),
        })
        .collect();

    let memory = crate::metrics::MemorySnapshot {
        total: sys.total_memory(),
        used: sys.used_memory(),
        available: sys.available_memory(),
        swap_total: sys.total_swap(),
        swap_used: sys.used_swap(),
    };

    let disks = disks
        .iter()
        .map(|disk| crate::metrics::DiskSnapshot {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            file_system: disk.file_system().to_string_lossy().to_string(),
            kind: format!("{:?}", disk.kind()),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        })
        .collect();

    crate::metrics::Metrics {
        global_cpu: sys.global_cpu_usage(),
        cpu_cores,
        memory,
        disks,
    }
}
