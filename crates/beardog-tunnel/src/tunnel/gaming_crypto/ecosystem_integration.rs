//! # Gaming Crypto Ecosystem Integration
//!
//! This module provides integration between gaming crypto systems and the
//! broader BearDog ecosystem, including performance optimization based on
//! network conditions and available resources.

use serde::{Deserialize, Serialize};

// ============================================================
// Optimization Context
// ============================================================

/// Context for ecosystem optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemOptimizationContext {
    /// Current network conditions
    pub network_conditions: NetworkConditions,

    /// Available system resources
    pub available_resources: ResourceAvailability,

    /// Optimization preferences
    pub preferences: OptimizationPreferences,
}

impl Default for EcosystemOptimizationContext {
    fn default() -> Self {
        Self {
            network_conditions: NetworkConditions::default(),
            available_resources: ResourceAvailability::default(),
            preferences: OptimizationPreferences::default(),
        }
    }
}

// ============================================================
// Performance Requirements
// ============================================================

/// Performance requirements for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    /// Maximum allowed latency in milliseconds
    pub max_latency_ms: f64,

    /// Minimum required throughput in Mbps
    pub min_throughput_mbps: f64,

    /// Maximum CPU utilization (0.0 - 1.0)
    pub max_cpu_utilization: f64,

    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
}

impl Default for PerformanceRequirements {
    fn default() -> Self {
        Self {
            max_latency_ms: 100.0,
            min_throughput_mbps: 100.0,
            max_cpu_utilization: 0.5,
            max_memory_bytes: 64 * 1024 * 1024, // 64MB
        }
    }
}

// ============================================================
// Network Conditions
// ============================================================

/// Current network conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConditions {
    /// Current latency in milliseconds
    pub current_latency_ms: f64,

    /// Current bandwidth in Mbps
    pub current_bandwidth_mbps: f64,

    /// Packet loss rate (0.0 - 1.0)
    pub packet_loss_rate: f64,

    /// Network stability score (0.0 - 1.0)
    pub stability_score: f64,
}

impl Default for NetworkConditions {
    fn default() -> Self {
        Self {
            current_latency_ms: 50.0,
            current_bandwidth_mbps: 1000.0,
            packet_loss_rate: 0.001,
            stability_score: 0.9,
        }
    }
}

// ============================================================
// Resource Availability
// ============================================================

/// Available system resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAvailability {
    /// Available CPU cores
    pub available_cpu_cores: u32,

    /// Available memory in bytes
    pub available_memory_bytes: u64,

    /// Available hardware acceleration features
    pub hardware_acceleration: Vec<String>,

    /// Available SIMD instruction sets
    pub simd_support: Vec<String>,
}

impl Default for ResourceAvailability {
    fn default() -> Self {
        Self {
            available_cpu_cores: 8,
            available_memory_bytes: 16 * 1024 * 1024 * 1024, // 16GB
            hardware_acceleration: vec!["AES-NI".to_string()],
            simd_support: vec!["SSE4.2".to_string(), "AVX2".to_string()],
        }
    }
}

// ============================================================
// Optimization Preferences
// ============================================================

/// Optimization preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationPreferences {
    /// Enable genetic algorithm optimization
    pub enable_genetic_optimization: bool,

    /// Use hardware acceleration when available
    pub use_hardware_acceleration: bool,

    /// Enable adaptive algorithm selection
    pub adaptive_selection: bool,
}

impl Default for OptimizationPreferences {
    fn default() -> Self {
        Self {
            enable_genetic_optimization: true,
            use_hardware_acceleration: true,
            adaptive_selection: true,
        }
    }
}

// ============================================================
// Optimization Result
// ============================================================

/// Result of ecosystem optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemOptimizationResult {
    /// Recommended algorithm
    pub recommended_algorithm: String,

    /// Optimized parameters
    pub optimized_parameters: Vec<u8>,

    /// Expected performance improvements
    pub expected_improvements: PerformanceImprovements,

    /// Contributing optimization modules
    pub contributing_modules: Vec<String>,

    /// Confidence score (0.0 - 1.0)
    pub confidence_score: f64,
}

impl Default for EcosystemOptimizationResult {
    fn default() -> Self {
        Self {
            recommended_algorithm: "ChaCha20Poly1305".to_string(),
            optimized_parameters: vec![],
            expected_improvements: PerformanceImprovements::default(),
            contributing_modules: vec!["GeneticOptimizer".to_string()],
            confidence_score: 0.85,
        }
    }
}

/// Performance improvements from optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImprovements {
    /// Latency reduction percentage
    pub latency_reduction_percent: f64,

    /// Throughput increase percentage
    pub throughput_increase_percent: f64,

    /// CPU reduction percentage
    pub cpu_reduction_percent: f64,

    /// Memory reduction percentage
    pub memory_reduction_percent: f64,
}

impl Default for PerformanceImprovements {
    fn default() -> Self {
        Self {
            latency_reduction_percent: 15.0,
            throughput_increase_percent: 25.0,
            cpu_reduction_percent: 10.0,
            memory_reduction_percent: 5.0,
        }
    }
}

// ============================================================
// Optimizer
// ============================================================

/// Ecosystem optimizer
#[derive(Debug, Clone)]
pub struct EcosystemOptimizer {
    /// Current context
    context: EcosystemOptimizationContext,

    /// Performance requirements
    requirements: PerformanceRequirements,
}

impl EcosystemOptimizer {
    /// Create a new optimizer
    pub fn new(
        context: EcosystemOptimizationContext,
        requirements: PerformanceRequirements,
    ) -> Self {
        Self {
            context,
            requirements,
        }
    }

    /// Optimize for current conditions
    pub fn optimize(&self) -> EcosystemOptimizationResult {
        let mut result = EcosystemOptimizationResult::default();

        // Select algorithm based on conditions
        if self.context.available_resources.hardware_acceleration.contains(&"AES-NI".to_string()) {
            result.recommended_algorithm = "AES-256-GCM".to_string();
        } else if self.context.network_conditions.current_latency_ms < 50.0 {
            result.recommended_algorithm = "ChaCha20Poly1305".to_string();
        } else {
            result.recommended_algorithm = "ChaCha20".to_string();
        }

        // Calculate improvements based on optimization
        if self.context.preferences.enable_genetic_optimization {
            result.expected_improvements.throughput_increase_percent *= 1.2;
            result.contributing_modules.push("GeneticAlgorithm".to_string());
        }

        if self.context.preferences.use_hardware_acceleration {
            result.expected_improvements.cpu_reduction_percent *= 1.5;
            result.contributing_modules.push("HardwareAcceleration".to_string());
        }

        result
    }

    /// Check if requirements are met
    pub fn requirements_met(&self) -> bool {
        self.context.network_conditions.current_latency_ms <= self.requirements.max_latency_ms
            && self.context.network_conditions.current_bandwidth_mbps
                >= self.requirements.min_throughput_mbps
    }
}

impl Default for EcosystemOptimizer {
    fn default() -> Self {
        Self::new(
            EcosystemOptimizationContext::default(),
            PerformanceRequirements::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_context() {
        let context = EcosystemOptimizationContext::default();
        assert_eq!(context.network_conditions.current_latency_ms, 50.0);
        assert!(context.preferences.enable_genetic_optimization);
    }

    #[test]
    fn test_performance_requirements_default() {
        let req = PerformanceRequirements::default();
        assert_eq!(req.max_latency_ms, 100.0);
        assert_eq!(req.min_throughput_mbps, 100.0);
    }

    #[test]
    fn test_optimizer_with_aes_ni() {
        let optimizer = EcosystemOptimizer::default();
        let result = optimizer.optimize();
        assert_eq!(result.recommended_algorithm, "AES-256-GCM");
    }

    #[test]
    fn test_requirements_met() {
        let optimizer = EcosystemOptimizer::default();
        assert!(optimizer.requirements_met());
    }

    #[test]
    fn test_performance_improvements_default() {
        let improvements = PerformanceImprovements::default();
        assert_eq!(improvements.latency_reduction_percent, 15.0);
        assert_eq!(improvements.throughput_increase_percent, 25.0);
    }
}
