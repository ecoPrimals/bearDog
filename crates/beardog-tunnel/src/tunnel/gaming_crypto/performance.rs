//! # Gaming Crypto Performance Types
//!
//! This module provides types for monitoring and analyzing
//! gaming cryptography performance metrics.

use serde::{Deserialize, Serialize};

// ============================================================
// Network Performance
// ============================================================

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformanceMetrics {
    /// Round-trip time in microseconds
    pub rtt_us: f64,

    /// Jitter in microseconds
    pub jitter_us: f64,

    /// Packet loss rate (0.0 - 1.0)
    pub packet_loss_rate: f64,

    /// Bandwidth utilization (0.0 - 1.0)
    pub bandwidth_utilization: f64,

    /// RTT variance in microseconds
    pub rtt_variance_us: f64,
}

impl Default for NetworkPerformanceMetrics {
    fn default() -> Self {
        Self {
            rtt_us: 50.0,
            jitter_us: 5.0,
            packet_loss_rate: 0.001,
            bandwidth_utilization: 0.5,
            rtt_variance_us: 10.0,
        }
    }
}

impl NetworkPerformanceMetrics {
    /// Create new metrics with specified RTT
    pub fn with_rtt(rtt_us: f64) -> Self {
        Self {
            rtt_us,
            ..Default::default()
        }
    }

    /// Check if network is congested
    pub fn is_congested(&self) -> bool {
        self.bandwidth_utilization > 0.8 || self.packet_loss_rate > 0.05
    }

    /// Calculate network quality score (0.0 - 1.0)
    pub fn quality_score(&self) -> f64 {
        let rtt_score = (1.0 - (self.rtt_us / 1000.0).min(1.0)).max(0.0);
        let jitter_score = (1.0 - (self.jitter_us / 100.0).min(1.0)).max(0.0);
        let loss_score = 1.0 - self.packet_loss_rate.min(1.0);
        let bandwidth_score = 1.0 - self.bandwidth_utilization.min(1.0);

        (rtt_score * 0.4 + jitter_score * 0.2 + loss_score * 0.3 + bandwidth_score * 0.1)
    }
}

// ============================================================
// Performance Analysis
// ============================================================

/// Performance analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    /// Encryption latency in microseconds
    pub encryption_latency_us: f64,

    /// Decryption latency in microseconds
    pub decryption_latency_us: f64,

    /// Throughput in Mbps
    pub throughput_mbps: f64,

    /// CPU utilization (0.0 - 1.0)
    pub cpu_utilization: f64,

    /// Memory usage in bytes
    pub memory_usage_bytes: u64,

    /// Overall performance score (0.0 - 1.0)
    pub performance_score: f64,
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

impl PerformanceAnalysis {
    /// Create new analysis with specified throughput
    pub fn with_throughput(throughput_mbps: f64) -> Self {
        Self {
            throughput_mbps,
            ..Default::default()
        }
    }

    /// Check if performance is acceptable
    pub fn is_acceptable(&self) -> bool {
        self.performance_score >= 0.7
    }

    /// Calculate total crypto latency
    pub fn total_crypto_latency_us(&self) -> f64 {
        self.encryption_latency_us + self.decryption_latency_us
    }

    /// Check if latency is within target
    pub fn is_latency_within_target(&self, target_us: f64) -> bool {
        self.total_crypto_latency_us() <= target_us
    }

    /// Get memory usage in MB
    pub fn memory_usage_mb(&self) -> f64 {
        self.memory_usage_bytes as f64 / (1024.0 * 1024.0)
    }
}

// ============================================================
// Performance Thresholds
// ============================================================

/// Performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum acceptable RTT in microseconds
    pub max_rtt_us: f64,

    /// Maximum acceptable jitter in microseconds
    pub max_jitter_us: f64,

    /// Maximum acceptable packet loss rate
    pub max_packet_loss_rate: f64,

    /// Maximum acceptable encryption latency in microseconds
    pub max_encryption_latency_us: f64,

    /// Minimum acceptable throughput in Mbps
    pub min_throughput_mbps: f64,

    /// Maximum acceptable CPU utilization
    pub max_cpu_utilization: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_rtt_us: 200.0,
            max_jitter_us: 50.0,
            max_packet_loss_rate: 0.05,
            max_encryption_latency_us: 200.0,
            min_throughput_mbps: 100.0,
            max_cpu_utilization: 0.8,
        }
    }
}

impl PerformanceThresholds {
    /// Check if network metrics meet thresholds
    pub fn check_network(&self, metrics: &NetworkPerformanceMetrics) -> bool {
        metrics.rtt_us <= self.max_rtt_us
            && metrics.jitter_us <= self.max_jitter_us
            && metrics.packet_loss_rate <= self.max_packet_loss_rate
    }

    /// Check if analysis meets thresholds
    pub fn check_analysis(&self, analysis: &PerformanceAnalysis) -> bool {
        analysis.encryption_latency_us <= self.max_encryption_latency_us
            && analysis.throughput_mbps >= self.min_throughput_mbps
            && analysis.cpu_utilization <= self.max_cpu_utilization
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_metrics_default() {
        let metrics = NetworkPerformanceMetrics::default();
        assert_eq!(metrics.rtt_us, 50.0);
        assert_eq!(metrics.packet_loss_rate, 0.001);
    }

    #[test]
    fn test_network_quality_score() {
        let metrics = NetworkPerformanceMetrics::default();
        let score = metrics.quality_score();
        assert!(score > 0.5);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_performance_analysis_default() {
        let analysis = PerformanceAnalysis::default();
        assert_eq!(analysis.encryption_latency_us, 80.0);
        assert!(analysis.is_acceptable());
    }

    #[test]
    fn test_performance_thresholds() {
        let thresholds = PerformanceThresholds::default();
        let metrics = NetworkPerformanceMetrics::default();
        assert!(thresholds.check_network(&metrics));
    }

    #[test]
    fn test_congestion_detection() {
        let mut metrics = NetworkPerformanceMetrics::default();
        assert!(!metrics.is_congested());

        metrics.bandwidth_utilization = 0.9;
        assert!(metrics.is_congested());
    }
}
