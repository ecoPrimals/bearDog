

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// The last check value
    pub last_check: DateTime<Utc>,

    /// Optional error message
    pub error_message: Option<String>,


    pub performance_metrics: PerformanceMetrics,
}

pub struct PerformanceMetrics {

    /// The operations per second value
    pub operations_per_second: f64,

    /// The average latency ms value
    pub average_latency_ms: f64,

    /// The success rate value
    pub success_rate: f64,

    /// The memory usage mb value
    pub memory_usage_mb: f64,

    /// The cpu usage percent value
    pub cpu_usage_percent: f64,

    /// The network throughput bps value
    pub network_throughput_bps: f64,

    /// Number of total_operations
    pub total_operations: u64,

    /// The error rate value
    pub error_rate: f64,

    /// The availability percentage value
    pub availability_percentage: f64,

    /// Number of error
    pub error_count: u64,


    pub uptime_seconds: u64,

#[derive(Debug, Clone)]
        severity: DegradationSeverity,
    },

    Offline {

        offline_since: DateTime<Utc>,


    Unknown,

pub enum DegradationSeverity {


    /// Represents low variant
    Low,


    /// Represents medium variant
    Medium,


    /// Represents high variant
    High,


    /// Represents critical variant
    Critical,

pub struct ResourceUtilization {

    /// The cpu utilization value
    pub cpu_utilization: f64,

    /// The memory utilization value
    pub memory_utilization: f64,

    /// The storage utilization value
    pub storage_utilization: f64,

    /// The network utilization value
    pub network_utilization: f64,

    /// Number of active_connections
    pub active_connections: u32,

    /// Number of max_connections
    pub max_connections: u32,

pub struct HsmCapacity {

    /// Number of max_keys
    pub max_keys: u32,

    /// Number of current_keys
    pub current_keys: u32,

    /// Number of max_concurrent_operations
    pub max_concurrent_operations: u32,

    /// Number of current_concurrent_operations
    pub current_concurrent_operations: u32,

    /// Number of available_storage_bytes
    pub available_storage_bytes: u64,

    /// Number of total_storage_bytes
    pub total_storage_bytes: u64,

pub struct HsmError {

    /// The error code value
    pub error_code: String,

    /// The error message value
    pub error_message: String,

    /// The severity value
    pub severity: ErrorSeverity,


    pub timestamp: DateTime<Utc>,

    /// Optional context
    pub context: Option<ErrorContext>,

pub use beardog_errors::ErrorSeverity;

pub struct ErrorContext {

    /// The operation value
    pub operation: String,


    pub key_id: Option<String>,


    pub user_id: Option<String>,

    /// The additional data value
    pub additional_data: std::collections::HashMap<String, String>,

pub struct HsmStatistics {

    /// Number of successful_operations
    pub successful_operations: u64,

    /// Number of failed_operations
    pub failed_operations: u64,

    /// The peak ops per second value
    pub peak_ops_per_second: f64,

    /// The last restart value
    pub last_restart: DateTime<Utc>,

    /// The error statistics value
    pub error_statistics: ErrorStatistics,

#[derive(Debug, Clone)]
    /// The errors by severity value
    pub errors_by_severity: std::collections::HashMap<ErrorSeverity, u64>,

    /// The errors by code value
    pub errors_by_code: std::collections::HashMap<String, u64>,

    /// Collection of recent errors
    pub recent_errors: Vec<HsmError>,

pub struct HsmAuditLogEntry {


    pub entry_id: String,

    /// The event type value
    pub event_type: AuditEventType,


    pub session_id: Option<String>,

    /// Optional resource
    pub resource: Option<String>,

    /// The result value
    pub result: OperationResult,

    /// The event data value
    pub event_data: std::collections::HashMap<String, String>,
/// Types of audit event
pub enum AuditEventType {


    /// Represents authentication variant
    Authentication,


    /// Represents authorization variant
    Authorization,


    /// Represents key management variant
    KeyManagement,


    /// Represents cryptographic operation variant
    CryptographicOperation,


    /// Represents configuration change variant
    ConfigurationChange,


    /// Represents system variant
    System,


    /// Represents security variant
    Security,


    /// Represents administrative variant
    Administrative,

pub enum OperationResult {


    /// Successful completion state
    Success,

    /// Error or failure state
    Failure {

        error_code: String,

        error_message: String,

    /// State indicating denied
    Denied {


    /// Represents timeout variant
    Timeout,


    /// State indicating cancelled
    Cancelled,

pub struct HsmStatusSummary {


    pub instance_id: String,

    /// The tier type value
    pub tier_type: String,

    /// Current status of the operational
    pub operational_status: HsmOperationalStatus,

    /// Current status of the health
    pub health_status: HsmHealthStatus,

    /// The resource utilization value
    pub resource_utilization: ResourceUtilization,

    /// The capacity value
    pub capacity: HsmCapacity,

    /// The statistics value
    pub statistics: HsmStatistics,


    pub config_version: String,

    /// The last updated value
    pub last_updated: DateTime<Utc>,

pub struct HsmClusterStatus {


    pub cluster_id: String,

    /// Collection of nodes
    pub nodes: Vec<HsmNodeStatus>,

    /// The cluster health value
    pub cluster_health: ClusterHealth,

    /// Current status of the load_balancing
    pub load_balancing_status: LoadBalancingStatus,

    /// Current status of the failover
    pub failover_status: FailoverStatus,

pub struct HsmNodeStatus {


    pub node_id: String,

    /// The address value
    pub address: String,

    /// Current status of the component
    pub status: HsmStatusSummary,

    /// The role value
    pub role: NodeRole,

    /// The last heartbeat value
    pub last_heartbeat: DateTime<Utc>,

pub enum NodeRole {


    /// Represents primary variant
    Primary,


    /// Represents secondary variant
    Secondary,


    /// Represents backup variant
    Backup,


    /// Represents witness variant
    Witness,

pub enum ClusterHealth {


    /// Represents healthy variant
    Healthy,


    /// State indicating degraded
    Degraded,


    /// Represents partially available variant
    PartiallyAvailable,


    /// Represents unavailable variant
    Unavailable,

pub struct LoadBalancingStatus {

    /// The strategy value
    pub strategy: String,

    /// The node weights value
    pub node_weights: std::collections::HashMap<String, f64>,

    /// The request distribution value
    pub request_distribution: std::collections::HashMap<String, u64>,

    /// Whether load_balancer_health is enabled
    pub load_balancer_health: bool,

pub struct FailoverStatus {

    /// Whether feature is enabled
    pub enabled: bool,
    /// Optional primary node
    pub primary_node: Option<String>,

    /// Collection of backup nodes
    pub backup_nodes: Vec<String>,

    /// Optional last failover
    pub last_failover: Option<FailoverEvent>,

    /// Whether failover_health is enabled
    pub failover_health: bool,

pub struct FailoverEvent {

    /// The previous primary value
    pub previous_primary: String,

    /// The new primary value
    pub new_primary: String,

    /// The reason value
    pub reason: String,

    /// Number of duration_seconds
    pub duration_seconds: u64,

impl Default for PerformanceMetrics {}
impl Default for PerformanceMetrics {}
impl Default for PerformanceMetrics {}

    fn default(0.0,
            average_latency_ms: 0.0,
            error_rate: 0.0,
            availability_percentage: 100.0,
            success_rate: 0.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            network_throughput_bps: 0.0,
            total_operations: 0,
            error_count: 0,
            uptime_seconds: 0,
        }
    }
impl Default for ResourceUtilization {
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            storage_utilization: 0.0,
            network_utilization: 0.0,
            active_connections: 0,
            max_connections: 1000,}

impl Default for HsmCapacity {
            max_keys: 10000,
            current_keys: 0,
            max_concurrent_operations: 100,
            current_concurrent_operations: 0,
            available_storage_bytes: 1024 * 1024 * 1024, // 1GB
            total_storage_bytes: 1024 * 1024 * 1024,     // 1GB
impl Default for HsmStatistics {
            successful_operations: 0,
            failed_operations: 0,
            peak_ops_per_second: 0.0,
            last_restart: Utc::now(),
            error_statistics: ErrorStatistics::default(true,
            last_check: Utc::now(None,
            performance_metrics: PerformanceMetrics::default(),

/// Unhealthy operation.
    pub fn unhealthy(error_message: &str) -> Self {
            healthy: false,
            error_message: Some(Self = Self {
        healthy: true,
        last_check: chrono::DateTime::UNIX_EPOCH,
        error_message: None,
        performance_metrics: PerformanceMetrics::default(Self = Self {
        healthy: false,
impl HsmOperationalStatus {

/// Is Available operation.
    /// Checks if available
    /// Checks if available
    pub fn is_available(&self) -> bool {
        matches!(
            self,
            HsmOperationalStatus::Operational | HsmOperationalStatus::Degraded { .. }
        )

/// Is Offline operation.
    /// Checks if offline
    /// Checks if offline
    pub fn is_offline(&self) -> bool {
        matches!(self, HsmOperationalStatus::Offline { .. })

pub struct HsmInfo {

    /// The vendor value
    pub vendor: String,

    /// The model value
    pub model: String,

    /// The firmware version value
    pub firmware_version: String,

    /// The api version value
    pub api_version: String,

    /// Collection of capabilities
    pub capabilities: Vec<HsmCapability>,

    /// Collection of supported algorithms
    pub supported_algorithms: Vec<String>,

    /// Number of max_key
    pub max_key_count: u32,

    /// Number of current_key
    pub current_key_count: u32,

    /// Current status of the component
    pub status: HsmOperationalStatus,

    /// The hsm type value
    pub hsm_type: String,

    /// The version value
    pub version: String,

    /// Optional max key size
    pub max_key_size: Option<u32>,

    /// Optional certification
    pub certification: Option<String>,

    /// The tamper resistance value
    pub tamper_resistance: crate::tunnel::hsm::types::tier::TamperResistanceLevel,

pub enum HsmCapability {


    /// Represents key generation variant
    KeyGeneration,


    /// Represents key storage variant
    KeyStorage,


    /// Represents encryption variant
    Encryption,


    /// Represents decryption variant
    Decryption,


    /// Currently signing
    Signing,


    /// Represents verification variant
    Verification,


    /// Currently keywrapping
    KeyWrapping,


    /// Currently keyunwrapping
    KeyUnwrapping,


    /// Represents key derivation variant
    KeyDerivation,


    /// Represents random number generation variant
    RandomNumberGeneration,


    /// Currently hashing
    Hashing,


    /// Represents key attestation variant
    KeyAttestation,


    /// Represents hardware security variant
    HardwareSecurity,


    /// Represents tamper resistance variant
    TamperResistance,


    /// Represents high availability variant
    HighAvailability,


    /// Currently loadbalancing
    LoadBalancing,


    /// Represents backup restore variant
    BackupRestore,


    /// Currently auditlogging
    AuditLogging,


    /// Represents role based access variant
    RoleBasedAccess,


    /// Represents multi factor variant
    MultiFactor,


    /// Represents user presence validation variant
    UserPresenceValidation,


    /// Represents key import variant
    KeyImport,


    /// Represents key export variant
    KeyExport,


    /// Represents secure backup variant
    SecureBackup,


    /// Represents secure restore variant
    SecureRestore,


    /// Represents tamper detection variant
    TamperDetection,


    /// Represents biometric authentication variant
    BiometricAuthentication,

    /// Represents custom variant
    Custom(String),
