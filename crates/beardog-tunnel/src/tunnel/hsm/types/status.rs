

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealthStatus {

    pub healthy: bool,

    pub last_check: DateTime<Utc>,

    pub error_message: Option<String>,

    pub performance_metrics: PerformanceMetrics,
}

pub struct PerformanceMetrics {

    pub operations_per_second: f64,

    pub average_latency_ms: f64,

    pub success_rate: f64,

    pub memory_usage_mb: f64,

    pub cpu_usage_percent: f64,

    pub network_throughput_bps: f64,

    pub total_operations: u64,

    pub error_rate: f64,

    pub availability_percentage: f64,

    pub error_count: u64,

    pub uptime_seconds: u64,

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmOperationalStatus {

    Operational,

    Starting,

    Shutting,

    Maintenance,

    Degraded {

        reason: String,

        severity: DegradationSeverity,
    },

    Offline {

        offline_since: DateTime<Utc>,

    Unknown,

pub enum DegradationSeverity {

    Low,

    Medium,

    High,

    Critical,

pub struct ResourceUtilization {

    pub cpu_utilization: f64,

    pub memory_utilization: f64,

    pub storage_utilization: f64,

    pub network_utilization: f64,

    pub active_connections: u32,

    pub max_connections: u32,

pub struct HsmCapacity {

    pub max_keys: u32,

    pub current_keys: u32,

    pub max_concurrent_operations: u32,

    pub current_concurrent_operations: u32,

    pub available_storage_bytes: u64,

    pub total_storage_bytes: u64,

pub struct HsmError {

    pub error_code: String,

    pub error_message: String,

    pub severity: ErrorSeverity,

    pub timestamp: DateTime<Utc>,

    pub context: Option<ErrorContext>,

pub use beardog_errors::ErrorSeverity;

pub struct ErrorContext {

    pub operation: String,

    pub key_id: Option<String>,

    pub user_id: Option<String>,

    pub additional_data: std::collections::HashMap<String, String>,

pub struct HsmStatistics {

    pub successful_operations: u64,

    pub failed_operations: u64,

    pub peak_ops_per_second: f64,

    pub last_restart: DateTime<Utc>,

    pub error_statistics: ErrorStatistics,

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorStatistics {

    pub total_errors: u64,

    pub errors_by_severity: std::collections::HashMap<ErrorSeverity, u64>,

    pub errors_by_code: std::collections::HashMap<String, u64>,

    pub recent_errors: Vec<HsmError>,

pub struct HsmAuditLogEntry {

    pub entry_id: String,

    pub event_type: AuditEventType,

    pub session_id: Option<String>,

    pub resource: Option<String>,

    pub result: OperationResult,

    pub event_data: std::collections::HashMap<String, String>,

pub enum AuditEventType {

    Authentication,

    Authorization,

    KeyManagement,

    CryptographicOperation,

    ConfigurationChange,

    System,

    Security,

    Administrative,

pub enum OperationResult {

    Success,

    Failure {

        error_code: String,

        error_message: String,

    Denied {

    Timeout,

    Cancelled,

pub struct HsmStatusSummary {

    pub instance_id: String,

    pub tier_type: String,

    pub operational_status: HsmOperationalStatus,

    pub health_status: HsmHealthStatus,

    pub resource_utilization: ResourceUtilization,

    pub capacity: HsmCapacity,

    pub statistics: HsmStatistics,

    pub config_version: String,

    pub last_updated: DateTime<Utc>,

pub struct HsmClusterStatus {

    pub cluster_id: String,

    pub nodes: Vec<HsmNodeStatus>,

    pub cluster_health: ClusterHealth,

    pub load_balancing_status: LoadBalancingStatus,

    pub failover_status: FailoverStatus,

pub struct HsmNodeStatus {

    pub node_id: String,

    pub address: String,

    pub status: HsmStatusSummary,

    pub role: NodeRole,

    pub last_heartbeat: DateTime<Utc>,

pub enum NodeRole {

    Primary,

    Secondary,

    Backup,

    Witness,

pub enum ClusterHealth {

    Healthy,

    Degraded,

    PartiallyAvailable,

    Unavailable,

pub struct LoadBalancingStatus {

    pub strategy: String,

    pub node_weights: std::collections::HashMap<String, f64>,

    pub request_distribution: std::collections::HashMap<String, u64>,

    pub load_balancer_health: bool,

pub struct FailoverStatus {

    pub enabled: bool,
    pub primary_node: Option<String>,

    pub backup_nodes: Vec<String>,

    pub last_failover: Option<FailoverEvent>,

    pub failover_health: bool,

pub struct FailoverEvent {

    pub previous_primary: String,

    pub new_primary: String,

    pub reason: String,

    pub duration_seconds: u64,

impl Default for PerformanceMetrics {}

    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
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
            error_statistics: ErrorStatistics::default(),

impl HsmHealthStatus {

    pub fn healthy() -> Self {
            healthy: true,
            last_check: Utc::now(),
            error_message: None,
            performance_metrics: PerformanceMetrics::default(),

    pub fn unhealthy(error_message: &str) -> Self {
            healthy: false,
            error_message: Some(error_message),

    pub const Healthy: Self = Self {
        healthy: true,
        last_check: chrono::DateTime::UNIX_EPOCH,
        error_message: None,
        performance_metrics: PerformanceMetrics::default(),
    };

    pub const Unknown: Self = Self {
        healthy: false,
impl HsmOperationalStatus {

    pub fn is_available(&self) -> bool {
        matches!(
            self,
            HsmOperationalStatus::Operational | HsmOperationalStatus::Degraded { .. }
        )

    pub fn is_offline(&self) -> bool {
        matches!(self, HsmOperationalStatus::Offline { .. })

pub struct HsmInfo {

    pub vendor: String,

    pub model: String,

    pub firmware_version: String,

    pub api_version: String,

    pub capabilities: Vec<HsmCapability>,

    pub supported_algorithms: Vec<String>,

    pub max_key_count: u32,

    pub current_key_count: u32,

    pub status: HsmOperationalStatus,

    pub hsm_type: String,

    pub version: String,

    pub max_key_size: Option<u32>,

    pub certification: Option<String>,

    pub tamper_resistance: crate::tunnel::hsm::types::tier::TamperResistanceLevel,

pub enum HsmCapability {

    KeyGeneration,

    KeyStorage,

    Encryption,

    Decryption,

    Signing,

    Verification,

    KeyWrapping,

    KeyUnwrapping,

    KeyDerivation,

    RandomNumberGeneration,

    Hashing,

    KeyAttestation,

    HardwareSecurity,

    TamperResistance,

    HighAvailability,

    LoadBalancing,

    BackupRestore,

    AuditLogging,

    RoleBasedAccess,

    MultiFactor,

    UserPresenceValidation,

    KeyImport,

    KeyExport,

    SecureBackup,

    SecureRestore,

    TamperDetection,

    BiometricAuthentication,

    Custom(String),
