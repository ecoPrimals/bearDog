// **BASE PROVIDER TRAITS**
//
// Core provider trait definitions that form the foundation of the unified provider system.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

///
/// This is the base trait that all providers must implement, providing
///
pub trait UnifiedProvider: Send + Sync {
    fn provider_info(&self) -> ProviderInfo;

    /// Get current provider health status
    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderHealth, BearDogError>> + Send;

    fn metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<ProviderMetrics, BearDogError>> + Send;

    /// Get provider capabilities
    fn capabilities(&self) -> Vec<ProviderCapability>;

    /// Initialize provider with configuration
    /// Initializes componentialize
    fn initialize(
        &mut self,
        config: ProviderConfiguration,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Shutdown provider gracefully
    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Unique provider identifier
    pub id: String,
    /// Human-readable provider name
    /// Name of the item
    pub name: String,
    /// Provider version
    /// The version value
    pub version: String,
    /// Provider type
    pub provider_type: ProviderType,
    /// Supported capabilities
    /// Collection of supported capabilities
    pub supported_capabilities: Vec<String>,
}

/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    /// Overall health status
    /// Current status of the component
    pub status: HealthStatus,
    /// Health check timestamp
    pub timestamp: SystemTime,
    /// Mapping of details
    pub details: HashMap<String, String>,
    /// The resource usage value
    pub resource_usage: ResourceUsage,
    /// Last error (if any)
    /// Optional last error
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetrics {
    /// Metrics collection timestamp
    pub timestamp: SystemTime,
    pub performance: HashMap<String, f64>,
    /// Custom metrics
    /// Collection of custom metrics
    pub custom_metrics: Vec<CustomMetric>,
    /// System metrics
    /// The system metrics value
    pub system_metrics: SystemMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage percentage (0.0 - 100.0)
    /// The cpu percent value
    pub cpu_percent: f64,
    /// Memory usage in bytes
    /// Number of `memory_bytes`
    pub memory_bytes: u64,
    /// Memory usage percentage (0.0 - 100.0)
    /// The memory percent value
    pub memory_percent: f64,
    /// Network I/O metrics
    /// The network io value
    pub network_io: NetworkIoMetrics,
    /// Disk I/O metrics
    /// Mapping of disk io
    pub disk_io: HashMap<String, u64>,
}

/// Provider capability description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapability {
    /// Capability name
    /// Name of the item
    pub name: String,
    /// Capability description
    /// The description value
    pub description: String,
    /// Capability parameters
    /// Collection of parameters
    pub parameters: Vec<CapabilityParameter>,
    /// Whether this capability is enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfiguration {
    /// Configuration parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Connection configuration
    /// The connection value
    pub connection: ConnectionConfiguration,
    /// Security configuration
    /// The security value
    pub security: SecurityConfiguration,
    pub performance: PerformanceConfiguration,
}

/// Provider type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of provider
pub enum ProviderType {
    /// Security provider (HSM, encryption, authentication)
    Security,
    /// Monitoring provider (metrics, logging, alerting)
    Monitoring,
    /// Storage provider (database, file system, object storage)
    Storage,
    /// Network provider (communication, service discovery)
    Network,
    /// AI provider (machine learning, inference)
    Ai,
    /// Generic provider type
    Generic,
    /// Custom provider type
    Custom(String),
}

/// Health status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Provider is healthy and operational
    Healthy,
    /// Provider is degraded but functional
    Degraded,
    /// Provider is unhealthy
    Unhealthy,
    /// Provider status is unknown
    Unknown,
}

/// Custom metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    /// Metric name
    /// Name of the item
    pub name: String,
    /// Metric value
    /// The value value
    pub value: f64,
    /// Metric unit
    /// The unit value
    pub unit: String,
    /// Metric description
    /// The description value
    pub description: String,
    /// Metric tags
    /// Mapping of tags
    pub tags: HashMap<String, String>,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Total requests processed
    /// Number of `total_requests`
    pub total_requests: u64,
    /// Successful requests
    /// Number of `successful_requests`
    pub successful_requests: u64,
    /// Failed requests
    /// Number of `failed_requests`
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Current active connections
    /// Number of `active_connections`
    pub active_connections: u32,
    /// Error rate (0.0 - 1.0)
    /// The error rate value
    pub error_rate: f64,
}

/// Network I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIoMetrics {
    /// Bytes sent
    /// Number of `bytes_sent`
    pub bytes_sent: u64,
    /// Bytes received
    /// Number of `bytes_received`
    pub bytes_received: u64,
    /// Packets sent
    /// Number of `packets_sent`
    pub packets_sent: u64,
    /// Packets received
    /// Number of `packets_received`
    pub packets_received: u64,
}

/// Capability parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityParameter {
    /// Parameter name
    /// Name of the item
    pub name: String,
    /// Parameter type
    /// The param type value
    pub param_type: String,
    /// Parameter description
    /// The description value
    pub description: String,
    /// Whether parameter is required
    /// Whether required is enabled
    pub required: bool,
    /// Default value (if any)
    /// Optional default value
    pub default_value: Option<serde_json::Value>,
}

/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfiguration {
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum retries
    /// Number of `max_retries`
    pub max_retries: u32,
    /// Connection pool size
    /// Number of `pool_size`
    pub pool_size: u32,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfiguration {
    /// Enable TLS encryption
    /// Whether tls is enabled
    pub tls_enabled: bool,
    /// Certificate path (if applicable)
    /// Optional cert path
    pub cert_path: Option<String>,
    /// Key path (if applicable)
    /// Optional key path
    pub key_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfiguration {
    /// Whether monitoring is enabled
    pub monitoring_enabled: bool,
    /// Metrics collection interval in seconds
    /// Number of `metrics_interval_seconds`
    pub metrics_interval_seconds: u64,
    /// The optimization level value
    pub optimization_level: String,
}
