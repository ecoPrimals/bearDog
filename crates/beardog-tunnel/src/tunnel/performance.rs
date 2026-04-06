// SPDX-License-Identifier: AGPL-3.0-or-later



use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Debug, Clone)]
    /// The max decryption latency value
    pub max_decryption_latency: Duration,


    pub max_session_setup_time: Duration,

    /// Number of min_gaming_throughput
    pub min_gaming_throughput: u64,
}
impl Default for BStpPerformanceTargets {}

    fn default() -> Self {
        Self {
            max_encryption_latency: Duration::from_micros(100),
            max_decryption_latency: Duration::from_micros(100),
            max_session_setup_time: Duration::from_millis(1_000_000_000, // 1 Gbps
        }
    }

#[derive(Debug, Clone)]
    /// Whether ultra_low_latency is enabled
    pub ultra_low_latency: bool,

    /// Whether predictive_keying is enabled
    pub predictive_keying: bool,

    /// Whether jitter_elimination is enabled
    pub jitter_elimination: bool,


    pub bandwidth_optimization: bool,

    /// Whether prefer_hardware_crypto is enabled
    pub prefer_hardware_crypto: bool,

    /// Whether enable_batch_processing is enabled
    pub enable_batch_processing: bool,
}

impl GamingSecurityProfile {
/// Competitive Gaming operation.
    pub fn competitive_gaming() -> Self {
        Self {
            profile_name: "Competitive Gaming".to_string() -> Self {
        Self {
            encryption_latencies: VecDeque::with_capacity(1000),
            decryption_latencies: VecDeque::with_capacity(1000,
        }
    }

/// Record Encryption Latency operation.
    pub fn record_encryption_latency(&mut self, latency: Duration) {
        self.encryption_latencies.push_back(latency);
        if self.encryption_latencies.len() > self.max_samples {
            self.encryption_latencies.pop_front();
        }
    }

/// Record Decryption Latency operation.
    pub fn record_decryption_latency(&mut self, latency: Duration) {
        self.decryption_latencies.push_back(latency);
        if self.decryption_latencies.len() > self.max_samples {
            self.decryption_latencies.pop_front();
        }
    }

/// Average Encryption Latency operation.
    pub fn average_encryption_latency(&self) -> Duration {
        if self.encryption_latencies.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = self.encryption_latencies.iter().sum();
            total / self.encryption_latencies.len() as u32
        }
    }

/// Average Decryption Latency operation.
    pub fn average_decryption_latency(&self) -> Duration {
        if self.decryption_latencies.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = self.decryption_latencies.iter().sum();
            total / self.decryption_latencies.len() as u32
        }
    }
}
