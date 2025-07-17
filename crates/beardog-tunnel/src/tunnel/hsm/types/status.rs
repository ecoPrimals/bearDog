use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// HSM health monitoring information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealthStatus {
    /// Whether the HSM is currently healthy and operational
    pub healthy: bool,
    /// Timestamp of the last health check
    pub last_check: DateTime<Utc>,
    /// Error message if health check failed
    pub error_message: Option<String>,
    /// Performance metrics for this HSM
    pub performance_metrics: PerformanceMetrics,
}

/// Performance metrics for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Operations performed per second
    pub operations_per_second: f64,
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    /// Error rate as a percentage (0.0 to 1.0)
    pub error_rate: f64,
    /// Availability percentage (0.0 to 100.0)
    pub availability_percentage: f64,
}

/// HSM operational status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmOperationalStatus {
    /// HSM is fully operational
    Operational,
    /// HSM is starting up
    Starting,
    /// HSM is shutting down
    Shutting,
    /// HSM is in maintenance mode
    Maintenance,
    /// HSM is degraded but still functional
    Degraded {
        /// Reason for degradation
        reason: String,
        /// Degradation severity
        severity: DegradationSeverity,
    },
    /// HSM is offline or unreachable
    Offline {
        /// Reason for being offline
        reason: String,
        /// Timestamp when HSM went offline
        offline_since: DateTime<Utc>,
    },
    /// HSM status is unknown
    Unknown,
}

/// Degradation severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DegradationSeverity {
    /// Low severity - minor performance impact
    Low,
    /// Medium severity - noticeable impact
    Medium,
    /// High severity - significant impact
    High,
    /// Critical severity - major functionality affected
    Critical,
}

/// HSM resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    /// CPU utilization percentage (0.0 to 100.0)
    pub cpu_utilization: f64,
    /// Memory utilization percentage (0.0 to 100.0)
    pub memory_utilization: f64,
    /// Storage utilization percentage (0.0 to 100.0)
    pub storage_utilization: f64,
    /// Network utilization percentage (0.0 to 100.0)
    pub network_utilization: f64,
    /// Number of active connections
    pub active_connections: u32,
    /// Maximum supported connections
    pub max_connections: u32,
}

/// HSM capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCapacity {
    /// Maximum number of keys that can be stored
    pub max_keys: u32,
    /// Current number of keys stored
    pub current_keys: u32,
    /// Maximum concurrent operations
    pub max_concurrent_operations: u32,
    /// Current concurrent operations
    pub current_concurrent_operations: u32,
    /// Available storage space in bytes
    pub available_storage_bytes: u64,
    /// Total storage space in bytes
    pub total_storage_bytes: u64,
}

/// HSM error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmError {
    /// Error code
    pub error_code: String,
    /// Error message
    pub error_message: String,
    /// Error severity
    pub severity: ErrorSeverity,
    /// Timestamp when error occurred
    pub timestamp: DateTime<Utc>,
    /// Additional error context
    pub context: Option<ErrorContext>,
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Informational message
    Info,
    /// Warning message
    Warning,
    /// Error message
    Error,
    /// Critical error
    Critical,
    /// Fatal error
    Fatal,
}

/// Error context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Operation that caused the error
    pub operation: String,
    /// Key ID involved (if applicable)
    pub key_id: Option<String>,
    /// User ID involved (if applicable)
    pub user_id: Option<String>,
    /// Additional context data
    pub additional_data: std::collections::HashMap<String, String>,
}

/// HSM statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmStatistics {
    /// Total operations performed
    pub total_operations: u64,
    /// Successful operations
    pub successful_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
    /// Average operation latency in milliseconds
    pub average_latency_ms: f64,
    /// Peak operations per second
    pub peak_ops_per_second: f64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Last restart time
    pub last_restart: DateTime<Utc>,
    /// Error statistics
    pub error_statistics: ErrorStatistics,
}

/// Error statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStatistics {
    /// Total errors
    pub total_errors: u64,
    /// Errors by severity
    pub errors_by_severity: std::collections::HashMap<ErrorSeverity, u64>,
    /// Errors by code
    pub errors_by_code: std::collections::HashMap<String, u64>,
    /// Recent errors (last 100)
    pub recent_errors: Vec<HsmError>,
}

/// HSM audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmAuditLogEntry {
    /// Unique audit entry ID
    pub entry_id: String,
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: AuditEventType,
    /// User ID (if applicable)
    pub user_id: Option<String>,
    /// Session ID (if applicable)
    pub session_id: Option<String>,
    /// Operation performed
    pub operation: String,
    /// Resource affected (e.g., key ID)
    pub resource: Option<String>,
    /// Operation result
    pub result: OperationResult,
    /// Additional event data
    pub event_data: std::collections::HashMap<String, String>,
}

/// Audit event types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEventType {
    /// Authentication event
    Authentication,
    /// Authorization event
    Authorization,
    /// Key management event
    KeyManagement,
    /// Cryptographic operation event
    CryptographicOperation,
    /// Configuration change event
    ConfigurationChange,
    /// System event
    System,
    /// Security event
    Security,
    /// Administrative event
    Administrative,
}

/// Operation result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationResult {
    /// Operation succeeded
    Success,
    /// Operation failed
    Failure {
        /// Error code
        error_code: String,
        /// Error message
        error_message: String,
    },
    /// Operation was denied
    Denied {
        /// Reason for denial
        reason: String,
    },
    /// Operation timed out
    Timeout,
    /// Operation was cancelled
    Cancelled,
}

/// HSM status summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmStatusSummary {
    /// HSM instance identifier
    pub instance_id: String,
    /// HSM tier type
    pub tier_type: String,
    /// Operational status
    pub operational_status: HsmOperationalStatus,
    /// Health status
    pub health_status: HsmHealthStatus,
    /// Resource utilization
    pub resource_utilization: ResourceUtilization,
    /// Capacity information
    pub capacity: HsmCapacity,
    /// Recent statistics
    pub statistics: HsmStatistics,
    /// Configuration version
    pub config_version: String,
    /// Last status update
    pub last_updated: DateTime<Utc>,
}

/// HSM cluster status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmClusterStatus {
    /// Cluster identifier
    pub cluster_id: String,
    /// Cluster nodes
    pub nodes: Vec<HsmNodeStatus>,
    /// Cluster health
    pub cluster_health: ClusterHealth,
    /// Load balancing status
    pub load_balancing_status: LoadBalancingStatus,
    /// Failover status
    pub failover_status: FailoverStatus,
}

/// HSM node status in a cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmNodeStatus {
    /// Node identifier
    pub node_id: String,
    /// Node address
    pub address: String,
    /// Node status
    pub status: HsmStatusSummary,
    /// Node role in cluster
    pub role: NodeRole,
    /// Last heartbeat
    pub last_heartbeat: DateTime<Utc>,
}

/// Node role in cluster
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeRole {
    /// Primary node
    Primary,
    /// Secondary node
    Secondary,
    /// Backup node
    Backup,
    /// Witness node
    Witness,
}

/// Cluster health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClusterHealth {
    /// All nodes healthy
    Healthy,
    /// Some nodes degraded
    Degraded,
    /// Cluster partially available
    PartiallyAvailable,
    /// Cluster unavailable
    Unavailable,
}

/// Load balancing status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingStatus {
    /// Load balancing strategy
    pub strategy: String,
    /// Node weights
    pub node_weights: std::collections::HashMap<String, f64>,
    /// Request distribution
    pub request_distribution: std::collections::HashMap<String, u64>,
    /// Load balancer health
    pub load_balancer_health: bool,
}

/// Failover status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverStatus {
    /// Failover enabled
    pub enabled: bool,
    /// Primary node
    pub primary_node: Option<String>,
    /// Backup nodes
    pub backup_nodes: Vec<String>,
    /// Last failover event
    pub last_failover: Option<FailoverEvent>,
    /// Failover health
    pub failover_health: bool,
}

/// Failover event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverEvent {
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Previous primary node
    pub previous_primary: String,
    /// New primary node
    pub new_primary: String,
    /// Failover reason
    pub reason: String,
    /// Failover duration in seconds
    pub duration_seconds: u64,
}

// Default implementations
impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            error_rate: 0.0,
            availability_percentage: 100.0,
        }
    }
}

impl Default for ResourceUtilization {
    fn default() -> Self {
        Self {
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            storage_utilization: 0.0,
            network_utilization: 0.0,
            active_connections: 0,
            max_connections: 1000,
        }
    }
}

impl Default for HsmCapacity {
    fn default() -> Self {
        Self {
            max_keys: 10000,
            current_keys: 0,
            max_concurrent_operations: 100,
            current_concurrent_operations: 0,
            available_storage_bytes: 1024 * 1024 * 1024, // 1GB
            total_storage_bytes: 1024 * 1024 * 1024,     // 1GB
        }
    }
}

impl Default for HsmStatistics {
    fn default() -> Self {
        Self {
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
            average_latency_ms: 0.0,
            peak_ops_per_second: 0.0,
            uptime_seconds: 0,
            last_restart: Utc::now(),
            error_statistics: ErrorStatistics::default(),
        }
    }
}

impl Default for ErrorStatistics {
    fn default() -> Self {
        Self {
            total_errors: 0,
            errors_by_severity: std::collections::HashMap::new(),
            errors_by_code: std::collections::HashMap::new(),
            recent_errors: Vec::new(),
        }
    }
}

impl HsmHealthStatus {
    /// Create a new healthy status
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            last_check: Utc::now(),
            error_message: None,
            performance_metrics: PerformanceMetrics::default(),
        }
    }

    /// Create a new unhealthy status
    pub fn unhealthy(error_message: String) -> Self {
        Self {
            healthy: false,
            last_check: Utc::now(),
            error_message: Some(error_message),
            performance_metrics: PerformanceMetrics::default(),
        }
    }
}

impl HsmOperationalStatus {
    /// Check if the HSM is available for operations
    pub fn is_available(&self) -> bool {
        matches!(self, HsmOperationalStatus::Operational | HsmOperationalStatus::Degraded { .. })
    }

    /// Check if the HSM is offline
    pub fn is_offline(&self) -> bool {
        matches!(self, HsmOperationalStatus::Offline { .. })
    }
}

/// HSM information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmInfo {
    /// HSM instance identifier
    pub instance_id: String,
    /// HSM tier type
    pub tier_type: String,
    /// HSM vendor information
    pub vendor: String,
    /// HSM model
    pub model: String,
    /// Firmware version
    pub firmware_version: String,
    /// API version
    pub api_version: String,
    /// Supported capabilities
    pub capabilities: Vec<HsmCapability>,
    /// Supported algorithms
    pub supported_algorithms: Vec<String>,
    /// Maximum key count
    pub max_key_count: u32,
    /// Current key count
    pub current_key_count: u32,
    /// HSM status
    pub status: HsmOperationalStatus,
}

/// HSM capability enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmCapability {
    /// Key generation capability
    KeyGeneration,
    /// Key storage capability
    KeyStorage,
    /// Encryption capability
    Encryption,
    /// Decryption capability
    Decryption,
    /// Digital signing capability
    Signing,
    /// Signature verification capability
    Verification,
    /// Key wrapping capability
    KeyWrapping,
    /// Key unwrapping capability
    KeyUnwrapping,
    /// Key derivation capability
    KeyDerivation,
    /// Random number generation capability
    RandomNumberGeneration,
    /// Hash computation capability
    Hashing,
    /// Key attestation capability
    KeyAttestation,
    /// Hardware security module capability
    HardwareSecurity,
    /// Tamper resistance capability
    TamperResistance,
    /// High availability capability
    HighAvailability,
    /// Load balancing capability
    LoadBalancing,
    /// Backup and restore capability
    BackupRestore,
    /// Audit logging capability
    AuditLogging,
    /// Role-based access control capability
    RoleBasedAccess,
    /// Multi-factor authentication capability
    MultiFactor,
    /// Custom capability
    Custom(String),
} 