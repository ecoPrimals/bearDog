// Universal Capability Types
//
// Type definitions for the universal capability-based adapter system.

use crate::ecosystem::primal_types::{DiscoveredPrimal, PrimalMetrics, UniversalEndpoint};
use beardog_types::canonical::capabilities::{
    AuthRequirements, CapabilityResponse, EndpointSecurityConfig, HealthStatus, PerformanceMetrics,
    ServiceCapabilityType, UniversalCapability,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AdapterConfig {
    /// Maximum number of providers to maintain per capability
    pub max_providers_per_capability: usize,
    /// Health check interval in seconds
    /// Number of health_check_interval_secs
    pub health_check_interval_secs: u64,
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    /// Number of max_retry_attempts
    pub max_retry_attempts: usize,
    /// Enable automatic failover to backup providers
    /// Whether enable_failover is enabled
    pub enable_failover: bool,
    /// The load balancing value
    pub load_balancing: LoadBalancingStrategy,
}

#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    /// Round-robin selection
    RoundRobin,
    PerformanceBased,
    /// Availability-based selection (highest uptime)
    AvailabilityBased,
}

#[derive(Debug, Clone)]
pub struct AdapterMetrics {
    /// Total capabilities discovered
    /// Number of total_capabilities_discovered
    pub total_capabilities_discovered: usize,
    /// Total primals connected
    /// Number of total_primals_connected
    pub total_primals_connected: usize,
    /// Average discovery time in milliseconds
    pub avg_discovery_time_ms: f64,
    /// The request success rate value
    pub request_success_rate: f64,
}

/// Active connection to a discovered capability
#[derive(Debug, Clone)]
pub struct CapabilityConnection {
    /// Capability identifier
    pub capability_id: String,
    /// Provider endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Connection status
    /// Current status of the component
    pub status: ConnectionStatus,
    /// Connection metrics
    /// The metrics value
    pub metrics: ConnectionMetrics,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    /// Connection is active and healthy
    Active,
    /// Connection is degraded but functional
    Degraded,
    /// Connection is failed
    Failed,
    /// Connection is being established
    Connecting,
}

#[derive(Debug, Clone)]
pub struct ConnectionMetrics {
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// The success rate value
    pub success_rate: f64,
    /// Last successful request timestamp
    pub last_success_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDiscoveryRequest {
    /// Required capability types
    /// Collection of required capabilities
    pub required_capabilities: Vec<ServiceCapabilityType>,
    /// Capability requirements and constraints
    /// The requirements value
    pub requirements: CapabilityRequirements,
    /// The preferences value
    pub preferences: CapabilityPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    /// Security requirements
    /// The security value
    pub security: SecurityRequirements,
    pub performance: PerformanceMetrics,
    /// Availability requirements
    /// The availability value
    pub availability: AvailabilityRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityPreferences {
    /// Preferred geographic regions
    /// Optional geographic constraints
    pub geographic_constraints: Option<GeographicConstraints>,
    /// Cost optimization preferences
    /// Whether cost_optimization is enabled
    pub cost_optimization: bool,
    pub performance_optimization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Minimum encryption level required
    /// The min encryption level value
    pub min_encryption_level: String,
    /// Authentication requirements
    /// The auth requirements value
    pub auth_requirements: AuthRequirements,
    /// Compliance requirements
    /// Collection of compliance standards
    pub compliance_standards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityRequirements {
    /// Minimum uptime percentage required
    pub min_uptime_percentage: f64,
    /// Maximum acceptable response time in milliseconds
    pub max_response_time_ms: u64,
    /// Fault tolerance requirements
    /// The fault tolerance level value
    pub fault_tolerance_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicConstraints {
    /// Preferred regions
    /// Collection of preferred regions
    pub preferred_regions: Vec<String>,
    /// Excluded regions
    /// Collection of excluded regions
    pub excluded_regions: Vec<String>,
}

/// Result of capability discovery operation
#[derive(Debug, Clone)]
pub struct CapabilityDiscoveryResult {
    /// Discovered and ranked capability providers
    pub providers: Vec<RankedCapabilityProvider>,
    /// Discovery metadata
    pub discovery_time_ms: u64,
    pub total_providers_found: usize,
}

#[derive(Debug, Clone)]
pub struct RankedCapabilityProvider {
    /// The capability provider
    pub provider: UniversalCapability,
    /// Ranking score (0.0 to 1.0, higher is better)
    /// The ranking score value
    pub ranking_score: f64,
    pub performance_estimate: PerformanceEstimate,
}

#[derive(Debug, Clone)]
pub struct PerformanceEstimate {
    /// Estimated response time in milliseconds
    pub estimated_response_time_ms: f64,
    /// Estimated throughput (requests per second)
    /// The estimated throughput rps value
    pub estimated_throughput_rps: f64,
    /// Confidence level in the estimate (0.0 to 1.0)
    pub confidence_level: f64,
}

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

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            max_providers_per_capability: 10,
            health_check_interval_secs: 30,
            connection_timeout_ms: 5000,
            max_retry_attempts: 3,
            enable_failover: true,
            load_balancing: LoadBalancingStrategy::PerformanceBased,
        }
    }
}

impl Default for AdapterMetrics {
    fn default() -> Self {
        Self {
            total_capabilities_discovered: 0,
            total_primals_connected: 0,
            avg_discovery_time_ms: 0.0,
            request_success_rate: 0.0,
        }
    }
}

impl Default for ConnectionMetrics {
    fn default() -> Self {
        Self {
            avg_response_time_ms: 0.0,
            success_rate: 0.0,
            last_success_timestamp: None,
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
