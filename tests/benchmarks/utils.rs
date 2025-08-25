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


//! Benchmark Utilities and Helper Functions
//!
//! Shared utilities for system info collection, performance analysis,
//! and recommendation generation.

use super::{BenchmarkResult, PerformanceMetrics, SystemInfo};

/// Collect system information
pub async fn collect_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        cpu_model: "Unknown CPU".to_string(),
        cpu_cores: num_cpus::get() as u32,
        memory_total_mb: 16384, // 16GB placeholder
        disk_type: "SSD".to_string(),
    }
}

/// Calculate performance grade based on results
pub fn calculate_performance_grade(results: &[BenchmarkResult], metrics: &PerformanceMetrics) -> String {
    let pass_count = results.iter().filter(|r| r.meets_threshold).count();
    let total_count = results.len();
    let pass_rate = pass_count as f64 / total_count as f64;

    let overall_ops_threshold = 500.0; // Minimum acceptable ops/sec
    let memory_threshold = 512.0; // Maximum acceptable memory usage in MB

    let performance_score = (pass_rate * 40.0) +
        (if metrics.overall_throughput_ops_per_sec >= overall_ops_threshold { 30.0 } else { 0.0 }) +
        (if metrics.peak_memory_usage_mb <= memory_threshold { 20.0 } else { 0.0 }) +
        (metrics.efficiency_score * 0.1);

    match performance_score as u32 {
        90..=100 => "A+ (Excellent)".to_string(),
        80..=89 => "A (Very Good)".to_string(),
        70..=79 => "B (Good)".to_string(),
        60..=69 => "C (Average)".to_string(),
        50..=59 => "D (Below Average)".to_string(),
        _ => "F (Poor)".to_string(),
    }
}

/// Generate performance recommendations based on results
pub fn generate_performance_recommendations(results: &[BenchmarkResult]) -> Vec<String> {
    let mut recommendations = Vec::new();

    // Analyze encryption performance
    let encryption_results: Vec<_> = results.iter()
        .filter(|r| r.name.starts_with("Encryption"))
        .collect();
    
    if encryption_results.iter().any(|r| !r.meets_threshold) {
        recommendations.push("Consider hardware acceleration for cryptographic operations".to_string());
        recommendations.push("Evaluate AES-NI or similar CPU optimizations".to_string());
    }

    // Analyze concurrency performance
    let concurrency_results: Vec<_> = results.iter()
        .filter(|r| r.name.starts_with("Concurrent"))
        .collect();

    if concurrency_results.len() > 1 {
        let max_ops = concurrency_results.iter().map(|r| r.operations_per_second).fold(0.0, f64::max);
        let min_ops = concurrency_results.iter().map(|r| r.operations_per_second).fold(f64::INFINITY, f64::min);
        let scalability_ratio = max_ops / min_ops;

        if scalability_ratio < 2.0 {
            recommendations.push("Limited scalability detected - consider async optimization".to_string());
            recommendations.push("Review lock contention and shared resource access patterns".to_string());
        }
    }

    // Analyze memory usage
    let high_memory_tests: Vec<_> = results.iter()
        .filter(|r| r.memory_usage_mb > 256.0)
        .collect();

    if !high_memory_tests.is_empty() {
        recommendations.push("High memory usage detected in some operations".to_string());
        recommendations.push("Consider implementing object pooling or zero-copy optimizations".to_string());
    }

    if recommendations.is_empty() {
        recommendations.push("Excellent performance across all benchmarks!".to_string());
        recommendations.push("Consider documenting current optimization strategies".to_string());
    }

    recommendations
} 