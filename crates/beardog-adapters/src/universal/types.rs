// Type definitions for the Universal Capability-Based Adapter
//
// This module contains all the type definitions used by the universal adapter
// to maintain clean separation of concerns and keep files under 1000 lines.

use crate::ecosystem::primal_types::UniversalEndpoint;
use beardog_types::canonical::capabilities::{
    EndpointSecurityConfig, HealthStatus, PerformanceMetrics, ServiceCapabilityType,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub max_retry_attempts: u32,
    /// Enable automatic failover
    /// Whether enable_failover is enabled
    pub enable_failover: bool,
    /// Load balancing strategy
    /// The load balancing value
    pub load_balancing: LoadBalancingStrategy,
}

#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    /// Represents round robin variant
    RoundRobin,
    /// Represents least connections variant
    LeastConnections,
    PerformanceBased,
    /// Represents random variant
    Random,
    /// State indicating healthbased
    HealthBased,
}

#[derive(Debug, Default)]
pub struct AdapterMetrics {
    /// Number of capabilities_discovered
    pub capabilities_discovered: u64,
    /// Number of primals_discovered
    pub primals_discovered: u64,
    /// Number of successful_connections
    pub successful_connections: u64,
    /// Number of failed_connections
    pub failed_connections: u64,
    /// Number of capability_requests
    pub capability_requests: u64,
    pub average_response_time_ms: f64,
    /// Number of active_connections
    pub active_connections: u64,
}

/// Active connection to a capability provider
#[derive(Debug)]
pub struct CapabilityConnection {
    pub provider_id: String,
    /// The capability type value
    pub capability_type: ServiceCapabilityType,
    /// The endpoint value
    pub endpoint: UniversalEndpoint,
    pub connection_id: String,
    /// The established at value
    pub established_at: std::time::SystemTime,
    /// The last health check value
    pub last_health_check: std::time::SystemTime,
    /// Current status of the health
    pub health_status: HealthStatus,
    /// The metrics value
    pub metrics: ConnectionMetrics,
}

/// Connection-specific metrics
#[derive(Debug, Default)]
pub struct ConnectionMetrics {
    /// Number of requests_sent
    pub requests_sent: u64,
    /// Number of responses_received
    pub responses_received: u64,
    /// Number of errors_enitemsered
    pub errors_encountered: u64,
    /// The average latency ms value
    pub average_latency_ms: f64,
    /// Optional last request at
    pub last_request_at: Option<std::time::SystemTime>,
}

/// Capability discovery request
#[derive(Debug, Clone)]
pub struct CapabilityDiscoveryRequest {
    /// The capability type value
    pub capability_type: ServiceCapabilityType,
    /// The requirements value
    pub requirements: CapabilityRequirements,
    /// Optional preferences
    pub preferences: Option<CapabilityPreferences>,
}

#[derive(Debug, Clone)]
pub struct CapabilityRequirements {
    /// The security value
    pub security: SecurityRequirements,
    /// The availability value
    pub availability: AvailabilityRequirements,
    pub performance: PerformanceMetrics,
}

#[derive(Debug, Clone)]
pub struct CapabilityPreferences {
    pub preferred_providers: Vec<String>,
    /// Optional geographic constraints
    pub geographic_constraints: Option<GeographicConstraints>,
    /// Whether cost_optimization is enabled
    pub cost_optimization: bool,
}

#[derive(Debug, Clone)]
pub struct SecurityRequirements {
    /// Number of min_security_level
    pub min_security_level: u8,
    /// Whether require_encryption is enabled
    pub require_encryption: bool,
    /// Whether require_authentication is enabled
    pub require_authentication: bool,
}

/// Availability requirements
#[derive(Debug, Clone)]
pub struct AvailabilityRequirements {
    pub min_uptime_percentage: f64,
    pub max_response_time_ms: u64,
}

/// Geographic constraints
#[derive(Debug, Clone)]
pub struct GeographicConstraints {
    /// Collection of allowed regions
    pub allowed_regions: Vec<String>,
    pub data_residency_requirements: Vec<String>,
}

/// Capability discovery result
#[derive(Debug)]
pub struct CapabilityDiscoveryResult {
    pub request_id: String,
    pub providers: Vec<RankedCapabilityProvider>,
    /// The selection criteria value
    pub selection_criteria: SelectionCriteria,
}

/// Ranked capability provider
#[derive(Debug)]
pub struct RankedCapabilityProvider {
    pub provider_id: String,
    /// The capability type value
    pub capability_type: ServiceCapabilityType,
    /// The rank value
    pub rank: f64,
    pub performance_estimate: PerformanceEstimate,
}

#[derive(Debug)]
pub struct PerformanceEstimate {
    /// The estimated latency ms value
    pub estimated_latency_ms: f64,
    /// The estimated throughput value
    pub estimated_throughput: f64,
    pub confidence_level: f64,
}

#[derive(Debug)]
pub struct SelectionCriteria {
    pub performance_weight: f64,
    /// The security weight value
    pub security_weight: f64,
    /// The availability weight value
    pub availability_weight: f64,
    /// The cost weight value
    pub cost_weight: f64,
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
