// SPDX-License-Identifier: AGPL-3.0-only

// Core Types for Advanced Metrics System

// Removed unused import: use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
// Removed incorrect imports added by migration script
// ServiceCapabilityType should be CapabilityType - the migration script made an error

#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    /// Metric name
    /// Name of the item
    pub name: String,
    /// Current value
    /// The value value
    pub value: f64,
    /// Collection of history
    pub history: Vec<MetricDataPoint>,
    /// Statistical summary
    /// The stats value
    pub stats: MetricStatistics,
    /// Metric type
    /// The metric type value
    pub metric_type: MetricType,
    /// Last updated timestamp
    /// The last updated value
    pub last_updated: SystemTime,
}

/// Security-related metric
#[derive(Debug, Clone)]
pub struct SecurityMetric {
    /// Security event type
    /// The event type value
    pub event_type: SecurityEventType,
    /// Event count
    /// Number of items
    pub count: u64,
    /// Severity distribution
    /// Mapping of severity distribution
    pub severity_distribution: HashMap<SecuritySeverity, u64>,
    /// Recent events
    /// Collection of recent events
    pub recent_events: Vec<SecurityEvent>,
    /// Threat level assessment
    /// The threat level value
    pub threat_level: ThreatLevel,
}

/// Ecosystem interaction metric
#[derive(Debug, Clone)]
pub struct EcosystemMetric {
    /// Service capability type (discovered dynamically, not hardcoded)
    /// The service value
    pub service: String,
    /// Interaction type
    /// The interaction type value
    pub interaction_type: InteractionType,
    /// Success rate
    /// The success rate value
    pub success_rate: f64,
    /// Average response time
    pub avg_response_time: Duration,
    /// Request volume
    /// Number of `request_volume`
    pub request_volume: u64,
    /// Error distribution
    /// Mapping of error distribution
    pub error_distribution: HashMap<String, u64>,
}

/// Custom application metric
#[derive(Debug, Clone)]
pub struct CustomMetric {
    /// Metric name
    /// Name of the item
    pub name: String,
    /// Metric value (can be numeric or text)
    /// The value value
    pub value: MetricValue,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
    /// Collection timestamp
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Metric value
    /// The value value
    pub value: f64,
    /// Optional context
    /// Optional context
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStatistics {
    /// Minimum value
    /// The min value
    pub min: f64,
    /// Maximum value
    /// The max value
    pub max: f64,
    /// Average value
    /// The avg value
    pub avg: f64,
    /// Standard deviation
    /// The std dev value
    pub std_dev: f64,
    /// 95th percentile
    /// The p95 value
    pub p95: f64,
    /// 99th percentile
    /// The p99 value
    pub p99: f64,
    /// Sample count
    /// Number of items
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricEvent {
    /// Event type
    /// The event type value
    pub event_type: MetricEventType,
    /// Metric name
    /// Name of the metric
    pub metric_name: String,
    /// Event data
    /// The data value
    pub data: serde_json::Value,
    /// Timestamp
    pub timestamp: SystemTime,
}

/// Security event details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event type
    /// The event type value
    pub event_type: SecurityEventType,
    /// Severity level
    /// The severity value
    pub severity: SecuritySeverity,
    /// Event description
    /// The description value
    pub description: String,
    /// Source component
    /// The source value
    pub source: String,
    /// Additional context
    /// Mapping of context
    pub context: HashMap<String, String>,
}

/// Cryptographic operation metric
#[derive(Debug, Clone)]
pub struct CryptoMetric {
    /// Operation type
    /// The operation value
    pub operation: String,
    /// Success count
    /// Number of success
    pub success_count: u64,
    /// Failure count
    /// Number of failure
    pub failure_count: u64,
    /// Average duration
    /// The avg duration value
    pub avg_duration: Duration,
    /// Key size distribution
    /// Mapping of key sizes
    pub key_sizes: HashMap<u32, u64>,
}

/// Trend prediction data point
#[derive(Debug, Clone)]
pub struct TrendPrediction {
    /// Predicted timestamp
    pub timestamp: u64,
    /// Predicted value
    /// The predicted value value
    pub predicted_value: f64,
    /// Confidence in prediction (0.0 to 1.0)
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct ThreatDetection {
    /// Detection timestamp
    pub timestamp: SystemTime,
    /// Threat type
    /// The threat type value
    pub threat_type: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Threat description
    /// The description value
    pub description: String,
    /// Affected components
    /// Collection of affected components
    pub affected_components: Vec<String>,
    /// Recommended actions
    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

/// HSM-specific metrics
#[derive(Debug, Clone)]
pub struct HsmMetrics {
    /// HSM provider name
    pub provider: String,
    /// Health status
    /// Current status of the health
    pub health_status: HsmHealthStatus,
    /// Operation counts
    /// Mapping of operations
    pub operations: HashMap<String, u64>,
    /// Average operation latency
    /// The avg latency value
    pub avg_latency: Duration,
    /// Error rate
    /// The error rate value
    pub error_rate: f64,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthStatus {
    /// Service name
    /// Name of the service
    pub service_name: String,
    /// Overall health status
    /// Current status of the component
    pub status: HealthStatus,
    /// Health score (0.0 to 1.0)
    /// The health score value
    pub health_score: f64,
    /// Last health check
    /// The last check value
    pub last_check: SystemTime,
    /// Status message
    /// The message value
    pub message: String,
}

/// Ecosystem-wide health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHealthStatus {
    /// Overall health status
    /// Current status of the overall
    pub overall_status: HealthStatus,
    /// Individual service statuses
    /// Mapping of services
    pub services: HashMap<String, ServiceHealthStatus>,
    /// System-wide metrics
    /// Mapping of system metrics
    pub system_metrics: HashMap<String, f64>,
    /// Active alerts count
    /// Number of `active_alerts`
    pub active_alerts: u32,
}

/// Metrics metadata
#[derive(Debug, Clone, Default)]
pub struct MetricsMetadata {
    /// Collection start time
    /// Optional collection start
    pub collection_start: Option<SystemTime>,
    /// Last update time
    /// Optional last update
    pub last_update: Option<SystemTime>,
    /// Total metrics collected
    /// Number of `total_metrics`
    pub total_metrics: u64,
    /// Collection version
    /// The version value
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSummary {
    /// Summary timestamp
    pub timestamp: SystemTime,
    pub performance_metrics_count: usize,
    /// Security events count
    /// Number of `security_events`
    pub security_events_count: usize,
    /// Ecosystem metrics count
    /// Number of `ecosystem_metrics`
    pub ecosystem_metrics_count: usize,
    /// Overall health score
    /// The overall health score value
    pub overall_health_score: f64,
}

// ENUMS

/// Types of metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of metric
pub enum MetricType {
    /// Represents counter variant
    Counter,
    /// Represents gauge variant
    Gauge,
    /// Represents histogram variant
    Histogram,
    /// Represents timer variant
    Timer,
    /// Represents rate variant
    Rate,
}

/// Security event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Types of security event
pub enum SecurityEventType {
    /// Represents authentication failure variant
    AuthenticationFailure,
    /// State indicating authorizationdenied
    AuthorizationDenied,
    /// Represents suspicious activity variant
    SuspiciousActivity,
    /// Represents data breach variant
    DataBreach,
    /// Represents intrusion attempt variant
    IntrusionAttempt,
    /// State indicating malwaredetected
    MalwareDetected,
    /// Represents unauthorized access variant
    UnauthorizedAccess,
    /// Represents policy violation variant
    PolicyViolation,
    /// Represents crypto failure variant
    CryptoFailure,
    /// Represents hsm tamper variant
    HsmTamper,
}

/// Security severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SecuritySeverity {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
    /// Represents emergency variant
    Emergency,
}

/// Threat level assessment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatLevel {
    /// No none specified
    None,
    /// Represents low variant
    Low,
    /// Represents moderate variant
    Moderate,
    /// Represents high variant
    High,
    /// Represents severe variant
    Severe,
    /// Represents critical variant
    Critical,
}

/// Types of ecosystem interactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of interaction
pub enum InteractionType {
    /// Represents api call variant
    ApiCall,
    /// Represents message queue variant
    MessageQueue,
    /// Represents database query variant
    DatabaseQuery,
    /// Represents file operation variant
    FileOperation,
    /// Represents network request variant
    NetworkRequest,
    /// Represents crypto operation variant
    CryptoOperation,
    /// Represents hsm operation variant
    HsmOperation,
}

/// Possible metric values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    /// Represents integer variant
    Integer(i64),
    /// Represents float variant
    Float(f64),
    /// Currently string
    String(String),
    /// Represents boolean variant
    Boolean(bool),
    /// Represents array variant
    Array(Vec<MetricValue>),
}

/// HSM health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HsmHealthStatus {
    /// Represents healthy variant
    Healthy,
    /// Currently warning
    Warning,
    /// Represents critical variant
    Critical,
    /// Represents offline variant
    Offline,
    /// Unknown or undefined state
    Unknown,
}

/// General health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Represents healthy variant
    Healthy,
    /// Currently warning
    Warning,
    /// Represents critical variant
    Critical,
    /// Unknown or undefined state
    Unknown,
}

/// Metric event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of metric event
pub enum MetricEventType {
    /// State indicating metricupdated
    MetricUpdated,
    /// State indicating thresholdexceeded
    ThresholdExceeded,
    /// State indicating anomalydetected
    AnomalyDetected,
    /// State indicating healthstatuschanged
    HealthStatusChanged,
    /// Represents security alert variant
    SecurityAlert,
}

/// Anomaly detection algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyAlgorithm {
    /// Represents statistical outlier variant
    StatisticalOutlier,
    /// Represents moving average variant
    MovingAverage,
    /// Currently exponentialsmoothing
    ExponentialSmoothing,
    /// Currently machinelearning
    MachineLearning,
}

/// Trend analysis algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendAlgorithm {
    /// Represents linear regression variant
    LinearRegression,
    /// Represents moving average variant
    MovingAverage,
    /// Currently exponentialsmoothing
    ExponentialSmoothing,
    /// Represents seasonal decomposition variant
    SeasonalDecomposition,
}
