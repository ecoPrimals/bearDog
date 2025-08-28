

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformanceMetrics {

    pub avg_latency_us: f64,

    pub jitter_us: f64,

    pub packet_loss_rate: f64,

    pub bandwidth_utilization: f64,

    pub rtt_variance_us: f64,
}

pub struct PerformanceAnalysis {
    pub encryption_latency_us: f64,
    pub decryption_latency_us: f64,
    pub throughput_mbps: f64,
    pub cpu_utilization: f64,
    pub memory_usage_bytes: u64,
    pub performance_score: f64,
}

impl Default for NetworkPerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_latency_us: 50.0,
            jitter_us: 5.0,
            packet_loss_rate: 0.001,
            bandwidth_utilization: 0.5,
            rtt_variance_us: 10.0,
        }
    }
}

impl Default for PerformanceAnalysis {
    fn default() -> Self {
        Self {
            encryption_latency_us: 80.0,
            decryption_latency_us: 75.0,
            throughput_mbps: 1000.0,
            cpu_utilization: 0.25,
            memory_usage_bytes: 1024 * 1024, // 1MB
            performance_score: 0.85,
        }
    }
}
