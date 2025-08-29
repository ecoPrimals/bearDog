use serde::{Deserialize, Serialize};

// SystemMetrics consolidated - using monitoring module
pub use crate::monitoring::SystemMetrics;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuMetrics {
    pub usage_percent: f64,

    pub cores: u32,

    pub temperature: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryMetrics {
    pub total_bytes: u64,

    pub used_bytes: u64,

    pub available_bytes: u64,

    pub usage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiskMetrics {
    pub total_bytes: u64,

    pub used_bytes: u64,

    pub available_bytes: u64,

    pub usage_percent: f64,

    pub read_ops_per_sec: f64,

    pub write_ops_per_sec: f64,
}

// NetworkMetrics using local definition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkMetrics {
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub packets_received: u64,
    pub packets_sent: u64,
    pub errors: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadAverage {
    pub one_minute: f64,

    pub five_minute: f64,

    pub fifteen_minute: f64,
}
