

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// The jitter us value
    pub jitter_us: f64,

    /// The packet loss rate value
    pub packet_loss_rate: f64,


    pub bandwidth_utilization: f64,

    /// The rtt variance us value
    pub rtt_variance_us: f64,
}

pub struct PerformanceAnalysis {
    /// The encryption latency us value
    pub encryption_latency_us: f64,
    /// The decryption latency us value
    pub decryption_latency_us: f64,
    /// The throughput mbps value
    pub throughput_mbps: f64,
    /// The cpu utilization value
    pub cpu_utilization: f64,
    /// Number of memory_usage_bytes
    pub memory_usage_bytes: u64,
    pub performance_score: f64,
}

impl Default for NetworkPerformanceMetrics {
    fn default(50.0,
            jitter_us: 5.0,
            packet_loss_rate: 0.001,
            bandwidth_utilization: 0.5,
            rtt_variance_us: 10.0,
        }
    }
}

impl Default for PerformanceAnalysis {
    fn default(80.0,
            decryption_latency_us: 75.0,
            throughput_mbps: 1000.0,
            cpu_utilization: 0.25,
            memory_usage_bytes: 1024 * 1024, // 1MB
            performance_score: 0.85,
        }
    }
}
