//! Extended Capability Types
//! 
//! Additional types for the universal capability-based adapter system.

use beardog_types::canonical::capabilities::{PerformanceMetrics, ServiceCapabilityType, UniversalCapability};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Security requirements for capability matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements { /// Whether TLS is required
    pub require_tls: bool,
    /// Required authentication method
    pub auth_method: String,
    /// Minimum security level (1-5)
    pub min_security_level: u8 }

impl Default for SecurityRequirements { fn default() -> Self {
        Self {
            require_tls: true,
            auth_method: "mutual_tls".to_string(),
            min_security_level: 3 }
    }
}

/// Availability requirements for capability selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityRequirements { /// Minimum uptime percentage (0.0-1.0)
    pub min_uptime: f64,
    /// Maximum acceptable response time in milliseconds
    pub max_response_time_ms: u64,
    /// Whether high availability is required
    pub require_high_availability: bool }

impl Default for AvailabilityRequirements { fn default() -> Self {
        Self {
            min_uptime: 0.99,
            max_response_time_ms: 100,
            require_high_availability: false }
    }
}

/// Geographic constraints for capability selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicConstraints { /// Preferred regions
    pub preferred_regions: Vec<String>,
    /// Excluded regions
    pub excluded_regions: Vec<String>,
    /// Whether data locality is required
    pub require_data_locality: bool }

impl Default for GeographicConstraints { fn default() -> Self {
        Self {
            preferred_regions: Vec::new(),
            excluded_regions: Vec::new(),
            require_data_locality: false }
    }
}

/// Result of capability discovery operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDiscoveryResult { /// Discovered capabilities ranked by suitability
    pub ranked_providers: Vec<RankedCapabilityProvider>,
    /// Total number of providers found
    pub total_providers: usize,
    /// Discovery operation duration in milliseconds
    pub discovery_time_ms: u64 }

/// A capability provider ranked by suitability score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedCapabilityProvider { /// The capability details
    pub capability: UniversalCapability,
    /// Suitability score (0.0-1.0, higher is better)
    pub suitability_score: f64,
    /// Performance estimate
    pub performance_estimate: PerformanceEstimate }

/// Performance estimates for a capability provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEstimate { /// Estimated response time in milliseconds
    pub estimated_response_time_ms: u64,
    /// Estimated throughput (operations per second)
    pub estimated_throughput_ops: u64,
    /// Confidence level in estimates (0.0-1.0)
    pub confidence_level: f64 }

/// Criteria for selecting capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionCriteria { /// Security requirements
    pub security: SecurityRequirements,
    /// Availability requirements
    pub availability: AvailabilityRequirements,
    /// Geographic constraints
    pub geographic: GeographicConstraints,
    /// Whether to prefer local capabilities
    pub prefer_local: bool }

impl Default for SelectionCriteria { fn default() -> Self {
        Self {
            security: SecurityRequirements::default(),
            availability: AvailabilityRequirements::default(),
            geographic: GeographicConstraints::default(),
            prefer_local: true }
    }
} 