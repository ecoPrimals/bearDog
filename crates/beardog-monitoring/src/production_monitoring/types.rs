//! # Production Monitoring Types
//!
//! This module provides types for production monitoring of the BearDog ecosystem,
//! including metrics, alerts, and health checking.

use beardog_genetics::{BiomeIdentity, GeneticSignature, TrustLevel};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ============================================================
// Enums
// ============================================================

/// Security log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLogLevel {
    /// Low security logging
    Low,
    /// Medium security logging
    Medium,
    /// High security logging
    High,
    /// Critical security logging
    Critical,
}

impl Default for SecurityLogLevel {
    fn default() -> Self {
        Self::Medium
    }
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Info level
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
    /// Critical level
    Critical,
}

impl Default for AlertSeverity {
    fn default() -> Self {
        Self::Info
    }
}

/// Alert type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    /// Performance alert
    Performance,
    /// Security alert
    Security,
    /// Availability alert
    Availability,
    /// Resource alert
    Resource,
    /// Genetic quality alert
    GeneticQuality,
}

impl Default for AlertType {
    fn default() -> Self {
        Self::Performance
    }
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Healthy
    }
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    /// Connected
    Connected,
    /// Disconnected
    Disconnected,
    /// Connecting
    Connecting,
    /// Error
    Error,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}

/// Security event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityEventType {
    /// Authentication event
    Authentication,
    /// Authorization event
    Authorization,
    /// Intrusion attempt
    Intrusion,
    /// Policy violation
    PolicyViolation,
    /// Anomaly detected
    Anomaly,
}

impl Default for SecurityEventType {
    fn default() -> Self {
        Self::Authentication
    }
}

/// Effort level for recommendations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffortLevel {
    /// Low effort
    Low,
    /// Medium effort
    Medium,
    /// High effort
    High,
}

impl Default for EffortLevel {
    fn default() -> Self {
        Self::Medium
    }
}

/// Priority level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Medium
    }
}

// ============================================================
// Configuration
// ============================================================

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Maximum retention hours for metrics
    pub max_retention_hours: u32,

    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,

    /// Health check interval in seconds
    pub health_check_interval_seconds: u32,

    /// Performance sampling rate (0.0 - 1.0)
    pub performance_sampling_rate: f64,

    /// Security log level
    pub security_log_level: SecurityLogLevel,

    /// Enable real-time analytics
    pub enable_real_time_analytics: bool,

    /// Dashboard refresh interval in seconds
    pub dashboard_refresh_seconds: u32,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            max_retention_hours: 24,
            alert_thresholds: AlertThresholds::default(),
            health_check_interval_seconds: 30,
            performance_sampling_rate: 0.1,
            security_log_level: SecurityLogLevel::Medium,
            enable_real_time_analytics: true,
            dashboard_refresh_seconds: 5,
        }
    }
}

/// Alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// Error rate maximum
    pub error_rate_maximum: f64,

    /// Genetic quality minimum
    pub genetic_quality_minimum: f64,

    /// Response time maximum in milliseconds
    pub response_time_max_ms: u64,

    /// Memory usage maximum percent
    pub memory_usage_max_percent: f64,

    /// CPU usage maximum percent
    pub cpu_usage_max_percent: f64,

    /// Biome connectivity minimum percent
    pub biome_connectivity_min_percent: f64,

    /// Error rate maximum percent
    pub error_rate_max_percent: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            error_rate_maximum: 0.05,
            genetic_quality_minimum: 0.7,
            response_time_max_ms: 1000,
            memory_usage_max_percent: 80.0,
            cpu_usage_max_percent: 85.0,
            biome_connectivity_min_percent: 90.0,
            error_rate_max_percent: 5.0,
        }
    }
}

// ============================================================
// Metrics Types
// ============================================================

/// Biome metrics
#[derive(Debug)]
pub struct BiomeMetrics {
    /// Biome identifier
    pub biome_id: String,

    /// Biome type
    pub biome_type: String,

    /// Trust level
    pub trust_level: TrustLevel,

    /// Total operations
    pub total_operations: AtomicU64,

    /// Successful operations
    pub successful_operations: AtomicU64,

    /// Failed operations
    pub failed_operations: AtomicU64,

    /// Average response time in milliseconds
    pub average_response_time_ms: AtomicU64,

    /// Genetic quality score (fixed-point)
    pub genetic_quality_score: AtomicU64,

    /// Last activity timestamp
    pub last_activity: parking_lot::Mutex<DateTime<Utc>>,

    /// Collaboration count
    pub collaboration_count: AtomicU64,

    /// Security incidents
    pub security_incidents: AtomicU64,
}

impl Clone for BiomeMetrics {
    fn clone(&self) -> Self {
        Self {
            biome_id: self.biome_id.clone(),
            biome_type: self.biome_type.clone(),
            trust_level: self.trust_level,
            total_operations: AtomicU64::new(self.total_operations.load(Ordering::Relaxed)),
            successful_operations: AtomicU64::new(self.successful_operations.load(Ordering::Relaxed)),
            failed_operations: AtomicU64::new(self.failed_operations.load(Ordering::Relaxed)),
            average_response_time_ms: AtomicU64::new(self.average_response_time_ms.load(Ordering::Relaxed)),
            genetic_quality_score: AtomicU64::new(self.genetic_quality_score.load(Ordering::Relaxed)),
            last_activity: parking_lot::Mutex::new(*self.last_activity.lock()),
            collaboration_count: AtomicU64::new(self.collaboration_count.load(Ordering::Relaxed)),
            security_incidents: AtomicU64::new(self.security_incidents.load(Ordering::Relaxed)),
        }
    }
}

/// System metrics
#[derive(Debug)]
pub struct SystemMetrics {
    /// Active biomes count
    pub active_biomes: AtomicUsize,

    /// Total authorizations
    pub total_authorizations: AtomicU64,

    /// Memory usage in bytes
    pub memory_usage_bytes: AtomicU64,

    /// CPU usage percent (fixed-point)
    pub cpu_usage_percent: AtomicU64,

    /// Network throughput in bps
    pub network_throughput_bps: AtomicU64,

    /// Uptime in seconds
    pub uptime_seconds: AtomicU64,

    /// System health score (fixed-point)
    pub system_health_score: AtomicU64,
}

impl Clone for SystemMetrics {
    fn clone(&self) -> Self {
        Self {
            active_biomes: AtomicUsize::new(self.active_biomes.load(Ordering::Relaxed)),
            total_authorizations: AtomicU64::new(self.total_authorizations.load(Ordering::Relaxed)),
            memory_usage_bytes: AtomicU64::new(self.memory_usage_bytes.load(Ordering::Relaxed)),
            cpu_usage_percent: AtomicU64::new(self.cpu_usage_percent.load(Ordering::Relaxed)),
            network_throughput_bps: AtomicU64::new(self.network_throughput_bps.load(Ordering::Relaxed)),
            uptime_seconds: AtomicU64::new(self.uptime_seconds.load(Ordering::Relaxed)),
            system_health_score: AtomicU64::new(self.system_health_score.load(Ordering::Relaxed)),
        }
    }
}

/// Genetic metrics
#[derive(Debug)]
pub struct GeneticMetrics {
    /// Spawning events
    pub spawning_events: AtomicU64,

    /// Evolution cycles
    pub evolution_cycles: AtomicU64,

    /// Average genetic quality (fixed-point)
    pub average_genetic_quality: AtomicU64,

    /// Genetic diversity index (fixed-point)
    pub genetic_diversity_index: AtomicU64,

    /// Consensus participation count
    pub consensus_participation: AtomicU64,
}

impl Clone for GeneticMetrics {
    fn clone(&self) -> Self {
        Self {
            spawning_events: AtomicU64::new(self.spawning_events.load(Ordering::Relaxed)),
            evolution_cycles: AtomicU64::new(self.evolution_cycles.load(Ordering::Relaxed)),
            average_genetic_quality: AtomicU64::new(self.average_genetic_quality.load(Ordering::Relaxed)),
            genetic_diversity_index: AtomicU64::new(self.genetic_diversity_index.load(Ordering::Relaxed)),
            consensus_participation: AtomicU64::new(self.consensus_participation.load(Ordering::Relaxed)),
        }
    }
}

/// Authorization metrics
#[derive(Debug)]
pub struct AuthorizationMetrics {
    /// Successful authorizations
    pub successful_authorizations: AtomicU64,

    /// Failed authorizations
    pub failed_authorizations: AtomicU64,

    /// Average authorization time in milliseconds
    pub average_authorization_time_ms: AtomicU64,

    /// Trust score distribution
    pub trust_score_distribution: parking_lot::Mutex<HashMap<String, u64>>,
}

impl Clone for AuthorizationMetrics {
    fn clone(&self) -> Self {
        Self {
            successful_authorizations: AtomicU64::new(self.successful_authorizations.load(Ordering::Relaxed)),
            failed_authorizations: AtomicU64::new(self.failed_authorizations.load(Ordering::Relaxed)),
            average_authorization_time_ms: AtomicU64::new(self.average_authorization_time_ms.load(Ordering::Relaxed)),
            trust_score_distribution: parking_lot::Mutex::new(self.trust_score_distribution.lock().clone()),
        }
    }
}

/// Performance metrics
#[derive(Debug)]
pub struct PerformanceMetrics {
    /// Memory pool utilization
    pub memory_pool_utilization: AtomicU64,

    /// SIMD operations
    pub simd_operations: AtomicU64,

    /// Lock contention events
    pub lock_contention_events: AtomicU64,

    /// GC events
    pub gc_events: AtomicU64,

    /// Average operation latency in nanoseconds
    pub average_operation_latency_ns: AtomicU64,
}

impl Clone for PerformanceMetrics {
    fn clone(&self) -> Self {
        Self {
            memory_pool_utilization: AtomicU64::new(self.memory_pool_utilization.load(Ordering::Relaxed)),
            simd_operations: AtomicU64::new(self.simd_operations.load(Ordering::Relaxed)),
            lock_contention_events: AtomicU64::new(self.lock_contention_events.load(Ordering::Relaxed)),
            gc_events: AtomicU64::new(self.gc_events.load(Ordering::Relaxed)),
            average_operation_latency_ns: AtomicU64::new(self.average_operation_latency_ns.load(Ordering::Relaxed)),
        }
    }
}

// ============================================================
// Dashboard Types
// ============================================================

/// Dashboard summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    /// System health score (0.0 - 1.0)
    pub system_health: f64,

    /// Active biomes count
    pub active_biomes: usize,

    /// Total operations
    pub total_operations: u64,

    /// Average response time in milliseconds
    pub average_response_time: u64,

    /// Average genetic quality
    pub genetic_quality_avg: f64,

    /// Authorization success rate
    pub authorization_success_rate: f64,
}

impl Default for DashboardSummary {
    fn default() -> Self {
        Self {
            system_health: 1.0,
            active_biomes: 0,
            total_operations: 0,
            average_response_time: 0,
            genetic_quality_avg: 1.0,
            authorization_success_rate: 1.0,
        }
    }
}

// ============================================================
// Alert Types
// ============================================================

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub id: String,

    /// Alert type
    pub alert_type: AlertType,

    /// Severity
    pub severity: AlertSeverity,

    /// Source component
    pub source_component: String,

    /// Message
    pub message: String,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Whether resolved
    pub resolved: bool,

    /// Resolution timestamp
    pub resolution_timestamp: Option<DateTime<Utc>>,

    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Default for Alert {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            alert_type: AlertType::default(),
            severity: AlertSeverity::default(),
            source_component: String::new(),
            message: String::new(),
            timestamp: Utc::now(),
            resolved: false,
            resolution_timestamp: None,
            metadata: HashMap::new(),
        }
    }
}

// ============================================================
// Health Check Types
// ============================================================

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Component name
    pub component: String,

    /// Health status
    pub status: HealthStatus,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Response time in milliseconds
    pub response_time_ms: u64,

    /// Details
    pub details: HashMap<String, String>,

    /// Score (0.0 - 1.0)
    pub score: f64,
}

impl Default for HealthCheckResult {
    fn default() -> Self {
        Self {
            component: String::new(),
            status: HealthStatus::Healthy,
            timestamp: Utc::now(),
            response_time_ms: 0,
            details: HashMap::new(),
            score: 1.0,
        }
    }
}

// ============================================================
// Issue and Recommendation Types
// ============================================================

/// Detected issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedIssue {
    /// Component
    pub component: String,

    /// Severity (0.0 - 1.0)
    pub severity: f64,

    /// Impact description
    pub impact_description: String,

    /// Suggested solutions
    pub suggested_solutions: Vec<String>,

    /// Detected at
    pub detected_at: DateTime<Utc>,
}

/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    /// Component
    pub component: String,

    /// Description
    pub description: String,

    /// Expected improvement
    pub expected_improvement: f64,

    /// Implementation effort
    pub implementation_effort: EffortLevel,

    /// Priority
    pub priority: Priority,

    /// Generated at
    pub generated_at: DateTime<Utc>,
}

// ============================================================
// Security Event Types
// ============================================================

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event ID
    pub id: String,

    /// Event type
    pub event_type: SecurityEventType,

    /// Severity
    pub severity: AlertSeverity,

    /// Source biome
    pub source_biome: Option<String>,

    /// Target biome
    pub target_biome: Option<String>,

    /// Description
    pub description: String,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Whether resolved
    pub resolved: bool,

    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Default for SecurityEvent {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: SecurityEventType::default(),
            severity: AlertSeverity::default(),
            source_biome: None,
            target_biome: None,
            description: String::new(),
            timestamp: Utc::now(),
            resolved: false,
            metadata: HashMap::new(),
        }
    }
}

// ============================================================
// Biome Info Types
// ============================================================

/// Connected biome info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedBiomeInfo {
    /// Biome identity
    pub biome_identity: BiomeIdentity,

    /// Registration time
    pub registration_time: DateTime<Utc>,

    /// Last seen
    pub last_seen: DateTime<Utc>,

    /// Connection status
    pub connection_status: ConnectionStatus,

    /// Performance history
    pub performance_history: Vec<PerformanceDataPoint>,

    /// Security score
    pub security_score: f64,

    /// Collaboration partners
    pub collaboration_partners: Vec<String>,
}

/// Performance data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Response time in milliseconds
    pub response_time_ms: u64,

    /// Success rate
    pub success_rate: f64,

    /// Genetic quality
    pub genetic_quality: f64,

    /// Resource usage
    pub resource_usage: f64,
}

// ============================================================
// Snapshot Types
// ============================================================

/// Metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// System metrics snapshot
    pub system_metrics: SystemMetricsSnapshot,

    /// Biome metrics by ID
    pub biome_metrics: HashMap<String, BiomeMetricsSnapshot>,

    /// Performance metrics snapshot
    pub performance_metrics: PerformanceMetricsSnapshot,
}

/// System metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricsSnapshot {
    /// Active biomes
    pub active_biomes: usize,

    /// Memory usage in bytes
    pub memory_usage_bytes: u64,

    /// CPU usage percent
    pub cpu_usage_percent: u64,

    /// Uptime in seconds
    pub uptime_seconds: u64,
}

impl Default for SystemMetricsSnapshot {
    fn default() -> Self {
        Self {
            active_biomes: 0,
            memory_usage_bytes: 0,
            cpu_usage_percent: 0,
            uptime_seconds: 0,
        }
    }
}

/// Biome metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeMetricsSnapshot {
    /// Operations per second
    pub operations_per_second: f64,

    /// Success rate
    pub success_rate: f64,

    /// Response time in milliseconds
    pub response_time_ms: u64,

    /// Genetic quality
    pub genetic_quality: f64,
}

impl Default for BiomeMetricsSnapshot {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            success_rate: 1.0,
            response_time_ms: 0,
            genetic_quality: 1.0,
        }
    }
}

/// Performance metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsSnapshot {
    /// Memory pool utilization
    pub memory_pool_utilization: u64,

    /// Average latency in nanoseconds
    pub average_latency_ns: u64,

    /// Lock contention events
    pub lock_contention_events: u64,
}

impl Default for PerformanceMetricsSnapshot {
    fn default() -> Self {
        Self {
            memory_pool_utilization: 0,
            average_latency_ns: 0,
            lock_contention_events: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_config_default() {
        let config = MonitoringConfig::default();
        assert_eq!(config.max_retention_hours, 24);
        assert_eq!(config.health_check_interval_seconds, 30);
        assert!(config.enable_real_time_analytics);
    }

    #[test]
    fn test_alert_thresholds_default() {
        let thresholds = AlertThresholds::default();
        assert_eq!(thresholds.genetic_quality_minimum, 0.7);
        assert_eq!(thresholds.response_time_max_ms, 1000);
    }

    #[test]
    fn test_health_status_default() {
        assert_eq!(HealthStatus::default(), HealthStatus::Healthy);
    }

    #[test]
    fn test_alert_severity_ordering() {
        assert!(AlertSeverity::Critical > AlertSeverity::Error);
        assert!(AlertSeverity::Error > AlertSeverity::Warning);
        assert!(AlertSeverity::Warning > AlertSeverity::Info);
    }

    #[test]
    fn test_dashboard_summary_default() {
        let summary = DashboardSummary::default();
        assert_eq!(summary.system_health, 1.0);
        assert_eq!(summary.active_biomes, 0);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
    }
}
