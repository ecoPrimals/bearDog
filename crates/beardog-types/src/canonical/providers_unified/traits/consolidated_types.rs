// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::SystemTime;

// **SUPPORTING TYPES** - Data structures used by provider traits

/// **HSM Status Information**
///
/// Comprehensive status information for Hardware Security Module operations.
/// Provides real-time visibility into HSM availability, security posture,
/// and operational health for monitoring and alerting systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmStatus {
    /// HSM availability status
    ///
    /// Indicates whether the HSM is currently available for cryptographic
    /// operations. `false` may indicate hardware failure, maintenance mode,
    /// or configuration issues requiring immediate attention.
    pub is_available: bool,

    /// Current security level designation
    ///
    /// Describes the active security level (e.g., "FIPS-140-2-Level-3",
    /// "Common-Criteria-EAL4+"). This affects which operations are permitted
    /// and compliance requirements that are satisfied.
    pub security_level: String,

    /// Number of keys currently stored in the HSM
    ///
    /// Active key count for capacity planning and security auditing.
    /// High key counts may impact performance and require key lifecycle
    /// management attention.
    pub key_count: usize,

    /// Overall health status description
    ///
    /// Human-readable health status (e.g., "Healthy", "Warning", "Critical").
    /// Provides operational teams with actionable health information for
    /// maintenance planning and incident response.
    pub health: String,
}

/// **System Performance Metrics**
///
/// Comprehensive system resource utilization metrics for performance
/// monitoring and capacity planning. All usage values are percentages
/// (0.0-100.0) unless otherwise specified.
///
/// ## Monitoring Guidelines
///
/// - CPU usage >80% sustained may indicate resource contention
/// - Memory usage >90% may cause performance degradation
/// - Disk usage >85% requires immediate attention
/// - Network I/O tracks bandwidth utilization trends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU utilization percentage (0.0-100.0)
    ///
    /// Represents current CPU usage across all cores. High sustained
    /// values indicate computational load that may affect cryptographic
    /// operation performance and system responsiveness.
    pub cpu_usage: f64,

    /// Memory utilization percentage (0.0-100.0)
    ///
    /// Current memory usage including buffers and caches. Critical for
    /// systems handling large key stores or high-volume operations.
    /// Memory pressure can significantly impact security operations.
    pub memory_usage: f64,

    /// Disk utilization percentage (0.0-100.0)
    ///
    /// Storage space utilization for logs, keys, and operational data.
    /// High disk usage can prevent proper logging and key storage,
    /// potentially affecting security and compliance requirements.
    pub disk_usage: f64,

    /// Network I/O utilization (Mbps)
    ///
    /// Current network bandwidth utilization in megabits per second.
    /// Important for distributed HSM operations and remote attestation
    /// scenarios where network performance affects security operations.
    pub network_io: f64,

    /// Timestamp when metrics were collected
    ///
    /// Precise timestamp for metric correlation and time-series analysis.
    /// Critical for performance trending and incident investigation.
    pub timestamp: std::time::SystemTime,
}

/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Unique alert identifier
    pub id: String,
    /// Alert severity level
    pub level: AlertLevel,
    /// Human-readable alert message
    pub message: String,
    /// Source component or service that generated the alert
    pub source: String,
    /// Timestamp when the alert was generated
    pub timestamp: std::time::SystemTime,
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AlertLevel {
    /// Informational alert - no action required
    Info,
    /// Warning alert - attention may be needed
    Warning,
    /// Error alert - action required
    Error,
    /// Critical alert - immediate action required
    Critical,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    /// Overall health status of the service
    pub status: HealthStatus,
    /// Detailed health check results
    pub checks: Vec<HealthCheckResult>,
    /// Timestamp of last health check update
    pub last_updated: std::time::SystemTime,
}

/// Health status of a service or component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Service is healthy and operational
    Healthy,
    /// Service is experiencing issues but still functional
    Unhealthy,
    /// Service status is unknown or cannot be determined
    Unknown,
}

/// Health check result containing status and diagnostic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Name of the service being checked
    pub name: String,
    /// Current health status
    pub status: HealthStatus,
    /// Optional diagnostic message
    pub message: Option<String>,
    /// Duration of the health check in milliseconds
    pub duration_ms: u64,
}

/// System event for monitoring and observability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    /// Unique identifier for the event
    pub id: String,
    /// Type of event (e.g., "`service_started`", "`error_occurred`")
    pub event_type: String,
    /// Event data as key-value pairs
    pub data: BTreeMap<String, serde_json::Value>,
    /// Timestamp when the event occurred
    pub timestamp: std::time::SystemTime,
}

/// Database query result containing rows and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseResult {
    /// Query result rows as key-value pairs
    pub rows: Vec<BTreeMap<String, serde_json::Value>>,
    /// Number of rows affected by the operation
    pub affected_rows: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// HTTP request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    /// Target URL for the request
    pub url: String,
    /// HTTP method (GET, POST, PUT, DELETE, etc.)
    pub method: String,
    /// Request headers as key-value pairs
    pub headers: BTreeMap<String, String>,
    /// Optional request body
    pub body: Option<Vec<u8>>,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
}

/// HTTP response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    /// HTTP status code
    pub status_code: u16,
    /// Response headers as key-value pairs
    pub headers: BTreeMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
    /// Response time in milliseconds
    pub duration_ms: u64,
}

/// Network connectivity status and metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    /// Whether the network connection is active
    pub is_connected: bool,
    /// Network latency in milliseconds (if available)
    pub latency_ms: Option<u64>,
    /// Network bandwidth in bits per second (if available)
    pub bandwidth_bps: Option<u64>,
    /// Error rate as a percentage (0.0 to 1.0)
    pub error_rate: f64,
}

/// Workflow definition containing steps and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    /// Unique workflow identifier
    pub id: String,
    /// Human-readable workflow name
    pub name: String,
    /// Ordered list of workflow steps
    pub steps: Vec<WorkflowStep>,
    /// Maximum execution time in seconds
    pub timeout_seconds: u64,
}

/// Individual step within a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Unique step identifier
    pub id: String,
    /// Human-readable step name
    pub name: String,
    /// Action to be performed
    pub action: String,
    /// Step parameters as key-value pairs
    pub parameters: BTreeMap<String, serde_json::Value>,
}

/// Workflow execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    /// Workflow identifier
    pub workflow_id: String,
    /// Final execution status
    pub status: WorkflowStatus,
    /// Workflow output data
    pub output: BTreeMap<String, serde_json::Value>,
    /// Total execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Workflow execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowStatus {
    /// Workflow is waiting to be executed
    Pending,
    /// Workflow is currently executing
    Running,
    /// Workflow completed successfully
    Completed,
    /// Workflow failed during execution
    Failed,
    /// Workflow was cancelled before completion
    Cancelled,
}

/// Workflow instance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstance {
    /// Unique instance identifier
    pub id: String,
    /// Workflow name
    pub name: String,
    /// Current execution status
    pub status: WorkflowStatus,
    /// Instance creation timestamp
    pub created_at: std::time::SystemTime,
    /// Last update timestamp
    pub updated_at: std::time::SystemTime,
}

/// Adapter information and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterInfo {
    /// Unique adapter identifier
    pub id: String,
    /// Human-readable adapter name
    pub name: String,
    /// Adapter version string
    pub version: String,
    /// List of supported capabilities
    pub capabilities: Vec<String>,
    /// Current adapter status
    pub status: AdapterStatus,
}

/// Adapter operational status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdapterStatus {
    /// Adapter is available for use
    Available,
    /// Adapter is connected and active
    Connected,
    /// Adapter is disconnected
    Disconnected,
    /// Adapter is in an error state
    Error,
}

/// Adapter connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConnection {
    /// Adapter identifier
    pub adapter_id: String,
    /// Connection identifier
    pub connection_id: String,
    /// Connection establishment timestamp
    pub established_at: std::time::SystemTime,
}

/// Adapter operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterRequest {
    /// Type of operation to perform
    pub operation_type: String,
    /// Operation parameters
    pub parameters: BTreeMap<String, serde_json::Value>,
    /// Operation timeout in milliseconds
    pub timeout_ms: u64,
}

/// Adapter operation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Response data (if successful)
    pub data: Option<serde_json::Value>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Operation duration in milliseconds
    pub duration_ms: u64,
}

/// Adapter capabilities and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterCapabilities {
    /// List of supported operations
    pub supported_operations: Vec<String>,
    /// Maximum number of concurrent operations
    pub max_concurrent_operations: usize,
    /// Whether streaming operations are supported
    pub supports_streaming: bool,
    /// Available security features
    pub security_features: Vec<String>,
}

/// Genetic algorithm population
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    /// Collection of individuals in the population
    pub individuals: Vec<Individual>,
    /// Current generation number
    pub generation: usize,
    /// Population fitness statistics
    pub fitness_stats: FitnessStats,
}

/// Individual organism in genetic algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual {
    /// Genetic representation (chromosome)
    pub genes: Vec<f64>,
    /// Fitness score (higher is better)
    pub fitness: Option<f64>,
    /// Age of the individual (generations survived)
    pub age: usize,
}

/// Evolution progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProgress {
    /// Current generation number
    pub generation: usize,
    /// Best fitness score in current generation
    pub best_fitness: f64,
    /// Average fitness score in current generation
    pub average_fitness: f64,
    /// Population diversity measure
    pub diversity: f64,
    /// Rate of fitness improvement
    pub convergence_rate: f64,
}

/// Fitness statistics for a population
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessStats {
    /// Best fitness score
    pub best: f64,
    /// Worst fitness score
    pub worst: f64,
    /// Average fitness score
    pub average: f64,
    /// Standard deviation of fitness scores
    pub standard_deviation: f64,
}

// Default implementations for common types
impl Default for HsmStatus {
    fn default() -> Self {
        Self {
            is_available: false,
            security_level: "unknown".to_string(),
            key_count: 0,
            health: "unknown".to_string(),
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_io: 0.0,
            timestamp: std::time::SystemTime::now(),
        }
    }
}

impl Default for ServiceHealth {
    fn default() -> Self {
        Self {
            status: HealthStatus::Unknown,
            checks: Vec::new(),
            last_updated: std::time::SystemTime::now(),
        }
    }
}

// **PROVIDER FRAMEWORK TYPES** - Supporting types for ConsolidatedProvider trait

/// Provider identification information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Unique provider identifier
    pub id: String,
    /// Human-readable provider name
    pub name: String,
    /// Provider version
    pub version: String,
    /// Provider type
    pub provider_type: ProviderType,
    /// Supported capabilities
    pub supported_capabilities: Vec<String>,
}

/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Health check timestamp
    pub last_check: SystemTime,
    /// Error message if unhealthy
    pub error_message: Option<String>,
    /// Provider uptime in seconds
    pub uptime_seconds: u64,
    /// Response time in milliseconds
    pub response_time_ms: u64,
}

/// Provider performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetrics {
    /// Metrics collection timestamp
    pub timestamp: SystemTime,
    /// Metric values
    pub metrics: BTreeMap<String, f64>,
    /// Metric metadata
    pub metadata: BTreeMap<String, String>,
}

/// Provider capability description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapability {
    /// Capability name
    pub name: String,
    /// Capability version
    pub version: String,
    /// Capability description
    pub description: String,
    /// Required configuration
    pub required_config: Vec<String>,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfiguration {
    /// Configuration parameters
    pub parameters: BTreeMap<String, serde_json::Value>,
    /// Configuration version
    pub version: String,
    /// Configuration metadata
    pub metadata: BTreeMap<String, String>,
}

/// Provider type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    /// Security provider
    Security,
    /// Cryptographic provider
    Crypto,
    /// HSM provider
    Hsm,
    /// Monitoring provider
    Monitoring,
    /// Storage provider
    Storage,
    /// Network provider
    Network,
    /// Genetics provider
    Genetics,
    /// Adapter provider
    Adapter,
    /// Workflow provider
    Workflow,
    /// Custom provider type
    Custom(String),
}

/// Authentication request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    /// User identifier
    pub user_id: String,
    /// Authentication method
    pub method: String,
    /// Authentication data
    pub data: BTreeMap<String, serde_json::Value>,
}

/// Authentication response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    /// Authentication success
    pub success: bool,
    /// Authentication token (if successful)
    pub token: Option<String>,
    /// Error message (if failed)
    pub error: Option<String>,
}

/// Authorization response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    /// Authorization granted
    pub authorized: bool,
    /// Granted permissions
    pub permissions: Vec<String>,
    /// Authorization context
    pub context: BTreeMap<String, String>,
}
