//! # Capability Adapter Types
//!
//! Type definitions and data structures for the Universal Capability Adapter.

use beardog_types::canonical::capabilities::{
    PerformanceMetrics, ServiceCapabilityType, UniversalCapability,
};
use crate::ecosystem::primal_types::DiscoveredPrimal;
use crate::universal::types::{AdapterConfig, AdapterMetrics, CapabilityConnection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security requirements for capability selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Whether require_tls is enabled
    pub require_tls: bool,
    /// The min tls version value
    pub min_tls_version: String,
    /// Whether require_mutual_auth is enabled
    pub require_mutual_auth: bool,
    /// Whether require_attestation is enabled
    pub require_attestation: bool,
    /// Collection of allowed cipher suites
    pub allowed_cipher_suites: Vec<String>,
}

/// Availability requirements for capability selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityRequirements {
    pub min_uptime_percentage: f64,
    pub max_response_time_ms: u64,
    /// Whether require_redundancy is enabled
    pub require_redundancy: bool,
    /// Whether maintenance_window_tolerance is enabled
    pub maintenance_window_tolerance: bool,
}

/// Geographic constraints for capability selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicConstraints {
    /// Collection of allowed regions
    pub allowed_regions: Vec<String>,
    /// Collection of prohibited regions
    pub prohibited_regions: Vec<String>,
    pub data_residency_requirements: Vec<String>,
}

/// Result of capability discovery operation
#[derive(Debug, Clone)]
pub struct CapabilityDiscoveryResult {
    pub request_id: String,
    pub discovered_providers: Vec<RankedCapabilityProvider>,
    /// Number of discovery_duration_ms
    pub discovery_duration_ms: u64,
    pub total_providers_found: usize,
    /// The selection criteria value
    pub selection_criteria: SelectionCriteria,
}

/// Capability provider with ranking information
#[derive(Debug, Clone)]
pub struct RankedCapabilityProvider {
    pub provider: UniversalCapability,
    /// The ranking score value
    pub ranking_score: f64,
    /// Collection of ranking reasons
    pub ranking_reasons: Vec<String>,
    pub estimated_performance: PerformanceEstimate,
}

/// Performance estimation for a capability provider
#[derive(Debug, Clone)]
pub struct PerformanceEstimate {
    /// Number of expected_latency_ms
    pub expected_latency_ms: u64,
    /// The expected throughput value
    pub expected_throughput: f64,
    /// The reliability score value
    pub reliability_score: f64,
    /// Optional cost estimate
    pub cost_estimate: Option<f64>,
}

/// Criteria for selecting capability providers
#[derive(Debug, Clone)]
pub struct SelectionCriteria {
    pub performance_weight: f64,
    /// The availability weight value
    pub availability_weight: f64,
    /// The security weight value
    pub security_weight: f64,
    /// The cost weight value
    pub cost_weight: f64,
    /// The locality weight value
    pub locality_weight: f64,
}

impl Default for SecurityRequirements {
    fn default() -> Self {
        Self {
            require_tls: true,
            min_tls_version: "1.3".to_string(),
            require_mutual_auth: false,
            require_attestation: true,
            allowed_cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
            ],
        }
    }
}

impl Default for AvailabilityRequirements {
    fn default() -> Self {
        Self {
            min_uptime_percentage: 99.9,
            max_response_time_ms: 100,
            require_redundancy: true,
            maintenance_window_tolerance: true,
        }
    }
}

impl Default for GeographicConstraints {
    fn default() -> Self {
        Self {
            allowed_regions: vec![],
            prohibited_regions: vec![],
            data_residency_requirements: vec![],
        }
    }
}

impl Default for SelectionCriteria {
    fn default() -> Self {
        Self {
            performance_weight: 0.3,
            availability_weight: 0.3,
            security_weight: 0.2,
            cost_weight: 0.1,
            locality_weight: 0.1,
        }
    }
} 