// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Performance Metrics and Analysis for Gaming Crypto
///
/// Provides performance monitoring and optimization metrics
/// for gaming-optimized cryptographic operations.

use serde::{Deserialize, Serialize};
/// Network performance metrics for gaming optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformanceMetrics {
    /// Average network latency in microseconds
    pub avg_latency_us: f64,
    /// Network jitter in microseconds
    pub jitter_us: f64,
    /// Packet loss rate (0.0 to 1.0)
    pub packet_loss_rate: f64,
    /// Bandwidth utilization (0.0 to 1.0)
    pub bandwidth_utilization: f64,
    /// Round-trip time variance
    pub rtt_variance_us: f64,
}
/// Performance analysis results
pub struct PerformanceAnalysis {
    /// Encryption latency in microseconds
    pub encryption_latency_us: f64,
    /// Decryption latency in microseconds
    pub decryption_latency_us: f64,
    /// Throughput in MB/s
    pub throughput_mbps: f64,
    /// CPU utilization (0.0 to 1.0)
    pub cpu_utilization: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Performance score (0.0 to 1.0, higher is better)
    pub performance_score: f64,}


impl Default for NetworkPerformanceMetrics {}


    fn default() -> Self {
        Self {
            avg_latency_us: 50.0,
            jitter_us: 5.0,
            packet_loss_rate: 0.001,
            bandwidth_utilization: 0.5,
            rtt_variance_us: 10.0,
        }
    }
impl Default for PerformanceAnalysis {
            encryption_latency_us: 80.0,
            decryption_latency_us: 75.0,
            throughput_mbps: 1000.0,
            cpu_utilization: 0.25,
            memory_usage_bytes: 1024 * 1024, // 1MB
            performance_score: 0.85,
