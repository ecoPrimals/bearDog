// 🛡️ BSTP Performance Monitoring

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Duration;

/// Performance targets for BSTP gaming tunnels  
#[derive(Debug, Clone)]
pub struct BStpPerformanceTargets {
    /// Maximum allowed encryption latency
    pub max_encryption_latency: Duration,
    /// Maximum allowed decryption latency
    pub max_decryption_latency: Duration,
    /// Maximum allowed session setup time
    pub max_session_setup_time: Duration,
    /// Minimum required gaming throughput in bytes per second
    pub min_gaming_throughput: u64,
}

impl Default for BStpPerformanceTargets {
    fn default() -> Self {
        Self {
            max_encryption_latency: Duration::from_micros(100),
            max_decryption_latency: Duration::from_micros(100),
            max_session_setup_time: Duration::from_millis(10),
            min_gaming_throughput: 1_000_000_000, // 1 Gbps
        }
    }
}

/// Gaming security profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingSecurityProfile {
    /// Name of the gaming security profile
    pub profile_name: String,
    /// Whether to enable ultra-low latency mode
    pub ultra_low_latency: bool,
    /// Whether to enable predictive keying
    pub predictive_keying: bool,
    /// Whether to enable jitter elimination
    pub jitter_elimination: bool,
    /// Whether to enable bandwidth optimization
    pub bandwidth_optimization: bool,
    /// Whether to prefer hardware crypto acceleration
    pub prefer_hardware_crypto: bool,
    /// Whether to enable batch processing of operations
    pub enable_batch_processing: bool,
}

impl GamingSecurityProfile {
    /// Create a competitive gaming security profile
    pub fn competitive_gaming() -> Self {
        Self {
            profile_name: "Competitive Gaming".to_string(),
            ultra_low_latency: true,
            predictive_keying: true,
            jitter_elimination: true,
            bandwidth_optimization: true,
            prefer_hardware_crypto: true,
            enable_batch_processing: true,
        }
    }
}

/// Latency monitoring
#[derive(Debug, Clone)]
pub struct LatencyMonitor {
    /// Rolling window of encryption latencies
    encryption_latencies: VecDeque<Duration>,
    /// Rolling window of decryption latencies
    decryption_latencies: VecDeque<Duration>,
    /// Maximum number of samples to keep
    max_samples: usize,
}

impl Default for LatencyMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl LatencyMonitor {
    /// Create a new latency monitor
    pub fn new() -> Self {
        Self {
            encryption_latencies: VecDeque::with_capacity(1000),
            decryption_latencies: VecDeque::with_capacity(1000),
            max_samples: 1000,
        }
    }

    /// Record a new encryption latency measurement
    pub fn record_encryption_latency(&mut self, latency: Duration) {
        self.encryption_latencies.push_back(latency);
        if self.encryption_latencies.len() > self.max_samples {
            self.encryption_latencies.pop_front();
        }
    }

    /// Record a new decryption latency measurement
    pub fn record_decryption_latency(&mut self, latency: Duration) {
        self.decryption_latencies.push_back(latency);
        if self.decryption_latencies.len() > self.max_samples {
            self.decryption_latencies.pop_front();
        }
    }

    /// Calculate average encryption latency from recorded samples
    pub fn average_encryption_latency(&self) -> Duration {
        if self.encryption_latencies.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = self.encryption_latencies.iter().sum();
            total / self.encryption_latencies.len() as u32
        }
    }

    /// Calculate average decryption latency from recorded samples
    pub fn average_decryption_latency(&self) -> Duration {
        if self.decryption_latencies.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = self.decryption_latencies.iter().sum();
            total / self.decryption_latencies.len() as u32
        }
    }
}
