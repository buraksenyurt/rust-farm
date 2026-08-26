use serde::Serialize;
use sysinfo::{Disks, System};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuCore {
    pub name: String,
    pub frequency: u64,
    pub usage: f32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemorySnapshot {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskSnapshot {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub kind: String,
    pub total_space: u64,
    pub available_space: u64,
    pub removable_space: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    pub global_cpu: f32,
    pub cpu_cores: Vec<CpuCore>,
    pub memory: MemorySnapshot,
    pub disks: Vec<DiskSnapshot>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HostInfo {
    pub name: String,
    pub os: String,
    pub kernel_version: String,
    pub architecture: String,
    pub cpu_brand: String,
    pub physical_cores: Option<usize>,
    pub logical_cores: usize,
}
