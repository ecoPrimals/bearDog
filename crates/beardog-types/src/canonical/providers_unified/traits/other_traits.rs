// SPDX-License-Identifier: AGPL-3.0-only

// **OTHER PROVIDER TRAITS**
//
// Monitoring, storage, network, and AI provider trait definitions.

use super::base_traits::UnifiedProvider;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// **MONITORING PROVIDER TRAIT** - For monitoring and observability providers
pub trait UnifiedMonitoringProvider: UnifiedProvider {
    /// Collect metrics from a source
    fn collect_metrics(
        &self,
        source: &str,
    ) -> impl std::future::Future<Output = Result<Vec<MetricData>, BearDogError>> + Send;

    /// Store metrics data
    fn store_metrics(
        &self,
        metrics: Vec<MetricData>,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Query metrics data
    fn query_metrics(
        &self,
        query: &str,
        start_time: SystemTime,
        end_time: SystemTime,
    ) -> impl std::future::Future<Output = Result<Vec<MetricData>, BearDogError>> + Send;

    /// Create an alert rule
    /// Creates alert
    fn create_alert(
        &self,
        rule: AlertRule,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// List active alerts
    fn list_alerts(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Alert>, BearDogError>> + Send;

    /// Log an event
    fn log_event(
        &self,
        event: LogEvent,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Query log events
    fn query_logs(
        &self,
        query: &str,
        start_time: SystemTime,
        end_time: SystemTime,
    ) -> impl std::future::Future<Output = Result<Vec<LogEvent>, BearDogError>> + Send;
}

/// **STORAGE PROVIDER TRAIT** - For data storage providers
pub trait UnifiedStorageProvider: UnifiedProvider {
    /// Store data
    fn store(
        &self,
        key: &str,
        data: &[u8],
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Retrieve data
    fn retrieve(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Delete data
    /// Removes
    fn delete(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// List stored keys
    fn list_keys(
        &self,
        prefix: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<String>, BearDogError>> + Send;

    /// Check if key exists
    fn exists(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    /// Get storage statistics
    fn storage_stats(
        &self,
    ) -> impl std::future::Future<Output = Result<StorageStats, BearDogError>> + Send;
}

/// **NETWORK PROVIDER TRAIT** - For network and communication providers
pub trait UnifiedNetworkProvider: UnifiedProvider {
    /// Send a message
    fn send_message(
        &self,
        message: NetworkMessage,
    ) -> impl std::future::Future<Output = Result<NetworkResponse, BearDogError>> + Send;

    /// Receive messages
    fn receive_messages(
        &self,
        timeout: Option<std::time::Duration>,
    ) -> impl std::future::Future<Output = Result<Vec<NetworkMessage>, BearDogError>> + Send;

    /// Register a service
    fn register_service(
        &self,
        registration: ServiceRegistration,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Discover services
    fn discover_services(
        &self,
        service_type: &str,
    ) -> impl std::future::Future<Output = Result<Vec<ServiceInfo>, BearDogError>> + Send;

    /// Check network connectivity
    fn check_connectivity(
        &self,
        target: &str,
    ) -> impl std::future::Future<Output = Result<ConnectivityResult, BearDogError>> + Send;

    /// Get network statistics
    fn network_stats(
        &self,
    ) -> impl std::future::Future<Output = Result<NetworkStats, BearDogError>> + Send;
}

/// **AI PROVIDER TRAIT** - For AI and machine learning providers
pub trait UnifiedAiProvider: UnifiedProvider {
    /// Process AI request
    /// Processes request
    fn process_request(
        &self,
        input: &str,
        model: &str,
    ) -> impl std::future::Future<Output = Result<AiResponse, BearDogError>> + Send;

    /// Load AI model
    /// Loads model
    fn load_model(
        &self,
        spec: ModelSpec,
    ) -> impl std::future::Future<Output = Result<ModelInfo, BearDogError>> + Send;

    /// Unload AI model
    fn unload_model(
        &self,
        model_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// List available models
    fn list_models(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<ModelInfo>, BearDogError>> + Send;

    /// Get model metrics
    fn model_metrics(
        &self,
        model_id: &str,
    ) -> impl std::future::Future<Output = Result<ModelMetrics, BearDogError>> + Send;

    /// Get AI statistics
    fn ai_stats(&self) -> impl std::future::Future<Output = Result<AiStats, BearDogError>> + Send;
}

// Supporting types for the provider traits

/// Metric data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricData {
    /// Metric name
    /// Name of the item
    pub name: String,
    /// Metric value
    /// The value value
    pub value: f64,
    /// Metric timestamp
    pub timestamp: SystemTime,
    /// Metric labels
    /// Mapping of labels
    pub labels: HashMap<String, String>,
    /// Metric unit
    /// The unit value
    pub unit: String,
}

/// Log event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Log level
    /// The level value
    pub level: LogLevel,
    /// Log message
    /// The message value
    pub message: String,
    /// Source component
    /// The source value
    pub source: String,
    /// Additional fields
    /// Mapping of fields
    pub fields: HashMap<String, String>,
}

/// Log level enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    /// Trace level
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
    /// Fatal level
    Fatal,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    /// Name of the item
    pub name: String,
    /// Rule description
    /// The description value
    pub description: String,
    /// Query expression
    /// The query value
    pub query: String,
    /// Alert severity
    /// The severity value
    pub severity: AlertSeverity,
    /// Evaluation interval
    /// The interval value
    pub interval: std::time::Duration,
    /// Alert threshold
    /// The threshold value
    pub threshold: f64,
    /// Alert labels
    /// Mapping of labels
    pub labels: HashMap<String, String>,
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub id: String,
    /// Alert rule name
    /// Name of the rule
    pub rule_name: String,
    /// Alert severity
    /// The severity value
    pub severity: AlertSeverity,
    /// Alert message
    /// The message value
    pub message: String,
    /// Alert timestamp
    pub timestamp: SystemTime,
    /// Alert labels
    /// Mapping of labels
    pub labels: HashMap<String, String>,
    /// Alert status
    /// Current status of the component
    pub status: String,
}

/// Alert severity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    /// Total storage capacity in bytes
    /// Number of `total_capacity_bytes`
    pub total_capacity_bytes: u64,
    /// Used storage in bytes
    /// Number of `used_bytes`
    pub used_bytes: u64,
    /// Available storage in bytes
    /// Number of `available_bytes`
    pub available_bytes: u64,
    /// Number of stored objects
    /// Number of object
    pub object_count: u64,
    /// Average object size in bytes
    /// Number of `avg_object_size_bytes`
    pub avg_object_size_bytes: u64,
}

/// Network message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    /// Message ID
    pub id: String,
    /// Source address
    /// The source value
    pub source: String,
    /// Destination address
    /// The destination value
    pub destination: String,
    /// Message payload
    /// Collection of payload
    pub payload: Vec<u8>,
    /// Message headers
    /// Mapping of headers
    pub headers: HashMap<String, String>,
    /// Message timestamp
    pub timestamp: SystemTime,
}

/// Network response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResponse {
    /// Response ID
    pub id: String,
    /// Response status
    /// Current status of the component
    pub status: NetworkStatus,
    /// Response payload
    /// Collection of payload
    pub payload: Vec<u8>,
    /// Response headers
    /// Mapping of headers
    pub headers: HashMap<String, String>,
    /// Response timestamp
    pub timestamp: SystemTime,
}

/// Network status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkStatus {
    /// Success
    Success,
    /// Error
    Error,
    /// Timeout
    Timeout,
    /// Connection refused
    ConnectionRefused,
    /// Network unreachable
    NetworkUnreachable,
}

/// Lightweight descriptor for a discovered network service endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service name
    /// Name of the item
    pub name: String,
    /// Service type
    /// The service type value
    pub service_type: String,
    /// Service address
    /// The address value
    pub address: String,
    /// Service port
    /// Number of port
    pub port: u16,
    /// Service metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Service registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    /// Service name
    /// Name of the item
    pub name: String,
    /// Service type
    /// The service type value
    pub service_type: String,
    /// Service address
    /// The address value
    pub address: String,
    /// Service port
    /// Number of port
    pub port: u16,
    /// Service metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Registration TTL
    /// Optional ttl
    pub ttl: Option<std::time::Duration>,
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Total bytes sent
    /// Number of `bytes_sent`
    pub bytes_sent: u64,
    /// Total bytes received
    /// Number of `bytes_received`
    pub bytes_received: u64,
    /// Total packets sent
    /// Number of `packets_sent`
    pub packets_sent: u64,
    /// Total packets received
    /// Number of `packets_received`
    pub packets_received: u64,
    /// Connection count
    /// Number of connection
    pub connection_count: u32,
    /// Error count
    /// Number of error
    pub error_count: u64,
}

/// Connectivity result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectivityResult {
    /// Target address
    /// The target value
    pub target: String,
    /// Connection successful
    /// Whether connected is enabled
    pub connected: bool,
    /// Response time
    pub response_time: Option<std::time::Duration>,
    /// Error message (if any)
    /// Optional error
    pub error: Option<String>,
}

/// AI response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    /// Response ID
    pub id: String,
    /// Model used
    /// The model value
    pub model: String,
    /// Response content
    /// The content value
    pub content: String,
    /// Response metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Processing time
    pub processing_time: std::time::Duration,
    /// Confidence score
    pub confidence: Option<f64>,
}

/// Model specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSpec {
    /// Model ID
    pub id: String,
    /// Model name
    /// Name of the item
    pub name: String,
    /// Model type
    /// The model type value
    pub model_type: String,
    /// Model source (file path, URL, etc.)
    /// The source value
    pub source: String,
    /// Model configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Published metadata for a loadable AI/ML model asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model ID
    pub id: String,
    /// Model name
    /// Name of the item
    pub name: String,
    /// Model type
    /// The model type value
    pub model_type: String,
    /// Model version
    /// The version value
    pub version: String,
    /// Model description
    /// The description value
    pub description: String,
    /// Model size in bytes
    /// Number of `size_bytes`
    pub size_bytes: u64,
    /// Model loaded status
    /// Whether loaded is enabled
    pub loaded: bool,
}

/// Model metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    /// Model ID
    pub model_id: String,
    /// Total requests processed
    /// Number of `total_requests`
    pub total_requests: u64,
    /// Successful requests
    /// Number of `successful_requests`
    pub successful_requests: u64,
    /// Failed requests
    /// Number of `failed_requests`
    pub failed_requests: u64,
    /// Average processing time
    pub avg_processing_time: std::time::Duration,
    /// Memory usage in bytes
    /// Number of `memory_usage_bytes`
    pub memory_usage_bytes: u64,
}

/// AI statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStats {
    /// Total models loaded
    /// Number of `models_loaded`
    pub models_loaded: u32,
    /// Total requests processed
    /// Number of `total_requests`
    pub total_requests: u64,
    /// Total processing time
    pub total_processing_time: std::time::Duration,
    /// Memory usage in bytes
    /// Number of `memory_usage_bytes`
    pub memory_usage_bytes: u64,
    /// GPU usage percentage
    /// Optional gpu usage percent
    pub gpu_usage_percent: Option<f64>,
}
