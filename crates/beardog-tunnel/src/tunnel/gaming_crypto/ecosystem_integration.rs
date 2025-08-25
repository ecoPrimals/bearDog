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


/// Ecosystem Integration for Gaming Crypto
///
/// Provides integration with the broader BearDog ecosystem for
/// distributed crypto optimization and capability discovery.

use serde::{Deserialize, Serialize};
/// Request for ecosystem-based crypto optimization
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcosystemOptimizationRequest {
    /// Target performance requirements
    pub performance_requirements: PerformanceRequirements,
    /// Current network conditions
    pub network_conditions: NetworkConditions,
    /// Available computational resources
    pub available_resources: ResourceAvailability,
    /// Optimization preferences
    pub preferences: OptimizationPreferences,
}
/// Performance requirements for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    /// Maximum acceptable latency in microseconds
    pub max_latency_us: f64,
    /// Minimum required throughput in MB/s
    pub min_throughput_mbps: f64,
    /// Maximum CPU utilization (0.0 to 1.0)
    pub max_cpu_utilization: f64,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
/// Current network conditions
pub struct NetworkConditions {
    /// Current latency in microseconds
    pub current_latency_us: f64,
    /// Current bandwidth in MB/s
    pub current_bandwidth_mbps: f64,
    /// Packet loss rate (0.0 to 1.0)
    pub packet_loss_rate: f64,
    /// Network stability score (0.0 to 1.0)
    pub stability_score: f64,
/// Available computational resources
pub struct ResourceAvailability {
    /// Available CPU cores
    pub available_cores: usize,
    /// Available memory in bytes
    pub available_memory_bytes: u64,
    /// Hardware acceleration available
    pub hardware_acceleration: Vec<String>,
    /// SIMD instruction sets available
    pub simd_support: Vec<String>,
/// Optimization preferences
pub struct OptimizationPreferences {
    /// Prefer low latency over throughput
    pub prioritize_latency: bool,
    /// Allow genetic algorithm optimization
    pub enable_genetic_optimization: bool,
    /// Use hardware acceleration when available
    pub use_hardware_acceleration: bool,
    /// Enable adaptive algorithm selection
    pub adaptive_selection: bool,
/// Result from ecosystem optimization
pub struct EcosystemOptimizationResult {
    /// Recommended crypto algorithm
    pub recommended_algorithm: String,
    /// Optimized parameters
    pub optimized_parameters: Vec<u8>,
    /// Expected performance improvements
    pub expected_improvements: PerformanceImprovements,
    /// Contributing ecosystem modules
    pub contributing_modules: Vec<String>,
    /// Confidence in recommendations (0.0 to 1.0)
    pub confidence_score: f64,
/// Expected performance improvements
pub struct PerformanceImprovements {
    /// Latency reduction percentage
    pub latency_reduction_percent: f64,
    /// Throughput increase percentage
    pub throughput_increase_percent: f64,
    /// CPU usage reduction percentage
    pub cpu_reduction_percent: f64,
    /// Memory usage reduction percentage
    pub memory_reduction_percent: f64,}


impl Default for PerformanceRequirements {}


    fn default() -> Self {
        Self {
            max_latency_us: 100.0,
            min_throughput_mbps: 100.0,
            max_cpu_utilization: 0.5,
            max_memory_bytes: 64 * 1024 * 1024, // 64MB
        }
    }
impl Default for NetworkConditions {
            current_latency_us: 50.0,
            current_bandwidth_mbps: 1000.0,
            packet_loss_rate: 0.001,
            stability_score: 0.9,}


impl Default for ResourceAvailability {
            available_cores: 8,
            available_memory_bytes: 16 * 1024 * 1024 * 1024, // 16GB
            hardware_acceleration: vec!["AES-NI".to_string()],
            simd_support: vec!["SSE4.2".to_string(), "AVX2".to_string()],
impl Default for OptimizationPreferences {
            prioritize_latency: true,
            enable_genetic_optimization: true,
            use_hardware_acceleration: true,
            adaptive_selection: true,}


impl Default for EcosystemOptimizationResult {
            recommended_algorithm: "ChaCha20Poly1305".to_string(),
            optimized_parameters: vec![],
            expected_improvements: PerformanceImprovements::default(),
            contributing_modules: vec!["GeneticOptimizer".to_string()],
            confidence_score: 0.85,
impl Default for PerformanceImprovements {
            latency_reduction_percent: 15.0,
            throughput_increase_percent: 25.0,
            cpu_reduction_percent: 10.0,
            memory_reduction_percent: 5.0,
