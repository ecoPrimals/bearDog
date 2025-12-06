

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::{{improved_results::*, BearDogError}};
use serde_json;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, info};

/// Monitor System Health Improved operation.
pub async fn monitor_system_health_improved(Vec<&str>,
    monitoring_config: &ImprovedMonitoringConfig,
) -> Result<ProcessingOutcome<SystemHealthResult, BearDogError>> {
    let _start_time = Instant::now();
    info!(
        "🔍 Monitoring {} system components with improved patterns",
        components.len()
    );

    let validation_result = validate_monitoring_config_improved(monitoring_config)?;
    if validation_result.status != ValidationStatus::Passed {
        return Err(BearDogError::invalid_input({}",
            validation_result
                .findings
                .iter()
                .map(|f| f.message.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }
    let mut health_results = Vec::new();
    let mut failed_items = Vec::new();
    let mut successful_count = 0;

    for component in components {
        match monitor_component_health(&component, monitoring_config) {
            Ok(health_result) => {
                health_results.push(health_result);
                successful_count += 1;
            }
            Err(e) => {
                let now = chrono::Utc::now();
                failed_items.push(ProcessingFailure {
                    item_index: failed_items.len(),
                    item_value: serde_json::json!(component),
                    error: e.to_string(),
                    error_code: "HEALTH_CHECK_FAILED".to_string(),
                    error_message: e.to_string(),
                    failed_at: now,
                });
        }
    let total_items = health_results.len({}/{} components healthy",
        successful_count, total_items
    let now = chrono::Utc::now();

    let summary_result = SystemHealthResult {
        component_name: "system_health_summary".to_string(),
        health_status: if failed_items.is_empty() {
            ComponentHealthStatus::Healthy
        } else {
            ComponentHealthStatus::Degraded
        },
        check_timestamp: chrono::Utc::now(),
        response_time: std::time::Duration::from_millis(ComponentMetrics {
            cpu_usage_percent: 0.0,
            memory_usage_percent: 0.0,
            disk_usage_percent: 0.0,
            network_latency_ms: 0.0,
            error_rate_percent: 0.0,
            uptime_seconds: 0,
        warnings: vec![],
        last_error: None,
    };
    Ok(summary_result,
        failures: failed_items.clone(ProcessingStatistics {
            total_items: total_items as usize,
            successful_items: successful_count,
            failed_items: failed_items.len(),
            success_rate: (successful_count as f64 / total_items as f64) * 100.0,
            avg_processing_time: std::time::Duration::from_millis(0,
            processing_rate_per_second: processing_rate,
            average_item_processing_time: std::time::Duration::from_millis(100),
        context: OperationContext {
            operation_id: uuid::Uuid::new_v4(now,
            completed_at: now,
            component: "beardog-monitoring".to_string(),
            initiator: "system".to_string(),
            metadata: std::collections::HashMap::with_capacity(16),
        metrics: OperationMetrics::default(health_results,
        failed_items,
        configuration: ProcessingConfiguration {
            name: "system_health_check".to_string(),
            description: "System health monitoring configuration".to_string(),
            max_retries: 2,
            timeout_seconds: 30,
            parallel_processing: true,
    })
}

/// Collect Metrics Improved operation.
pub async fn collect_metrics_improved(Vec<MetricSource>,
    collection_config: &MetricCollectionConfig,
) -> Result<ProcessingOutcome<MetricCollectionResult, BearDogError>> {
    let total_sources = metric_sources.len() as u64;
        "📊 Collecting metrics from {} sources with improved patterns",
        total_sources
    let mut collection_results = Vec::new();
    for source in metric_sources {
        match collect_source_metrics(&source, collection_config) {
            Ok(result) => {
                collection_results.push(result);
                    item_value: serde_json::json!(source.source_id),
                    item_id: source.source_id,
                    error_code: "METRIC_COLLECTION_FAILED".to_string();
    for alert in alerts {
        match process_individual_alert(&alert, alert_config) {
                processing_results.push(result);
                    item_value: serde_json::json!(alert.alert_id),
                    item_id: alert.alert_id,
                    error_code: "ALERT_PROCESSING_FAILED".to_string()],
        notifications_sent: successful_count,
        escalation_triggered: false,
        incident_created: false,
        suppressed: false,
            total_items: total_alerts as usize,
            success_rate: (successful_count as f64 / total_alerts as f64) * 100.0,
            avg_processing_time: std::time::Duration::from_millis(60),
            average_item_processing_time: std::time::Duration::from_millis(processing_results,
            name: "alert_processing".to_string(),
            description: "Alert processing configuration".to_string(),
            max_retries: 1,
            timeout_seconds: 10,

/// Validates monitoring_config_improved
fn validate_monitoring_config_improved(&ImprovedMonitoringConfig,
) -> Result<ValidationOutcome, BearDogError> {
    debug!(
        "🔍 Validating monitoring configuration: {}",
        config.config_id
    let mut findings = Vec::new();
    let mut score: f64 = 1.0;

    if config.config_id.is_empty(FindingSeverity::Critical,
            field: "config.config_id".to_string(),
            message: "Monitoring configuration ID cannot be empty".to_string(),
            config_path: Some("config.config_id".to_string()),
            suggested_fix: Some("Provide a valid configuration ID".to_string()),
            suggestion: Some("Provide a valid configuration ID".to_string()),
            code: "MON001".to_string(),
            field: "config.collection_interval_seconds".to_string(),
            message: "Collection interval is very short - may impact performance".to_string(),
            config_path: Some("config.collection_interval_seconds".to_string()),
            suggested_fix: Some(
                "Consider increasing collection interval to at least 5 seconds"),
            suggestion: Some(
            code: "MON002".to_string(),
            field: "config.retention_days".to_string(),
            message: "Retention period must be at least 1 day".to_string(),
            config_path: Some("config.retention_days".to_string()),
            suggested_fix: Some("Set retention period to at least 1 day".to_string()),
            suggestion: Some("Set retention period to at least 1 day".to_string()),
            code: "MON003".to_string(),
        score -= 0.3;

    if config.cpu_threshold_percent > 100.0 || config.cpu_threshold_percent < 0.0 {
            field: "config.cpu_threshold_percent".to_string(),
            message: "CPU threshold must be between 0 and 100 percent".to_string(),
            config_path: Some("config.cpu_threshold_percent".to_string()),
            suggested_fix: Some("Set CPU threshold between 0 and 100".to_string()),
            suggestion: Some("Set CPU threshold between 0 and 100".to_string()),
            code: "MON004".to_string(),
        score -= 0.2;
    let valid = findings
        .iter()
        .all(|f| f.severity != FindingSeverity::Critical);
    let criteria = vec![
        ValidationCriteria {
            name: "Configuration Structure".to_string(),
            description: "Basic configuration structure and required fields".to_string(),
            passed: !findings
                .any(|f| f.field.starts_with("config") && f.severity == FindingSeverity::Critical),
            name: "Collection Settings".to_string(),
            description: "Metric collection interval and retention settings".to_string(0.3,
            passed: config.collection_interval_seconds >= 5 && config.retention_days >= 1,
            name: "Alert Thresholds".to_string(),
            description: "Alert threshold configuration and limits".to_string()",
        config.config_id, score
    Ok(if valid {
            ValidationStatus::Passed
            ValidationStatus::Failed
        criteria,
            started_at: chrono::Utc::now(),
            completed_at: chrono::Utc::now(&str,
    _config: &ImprovedMonitoringConfig,
) -> Result<SystemHealthResult, BearDogError> {
    debug!("🔍 Monitoring component health: {}", component);

    let health_status = match component {
        "database" => {

        "cache" => {

        "api_server" => {

        "hsm" => {

        _ => {

            ComponentHealthStatus::Unknown
    let metrics = ComponentMetrics {
        cpu_usage_percent: 45.2,
        memory_usage_percent: 67.8,
        disk_usage_percent: 23.1,
        network_latency_ms: 12.5,
        error_rate_percent: 0.1,
        uptime_seconds: 86400,
    let warnings = match health_status {
        ComponentHealthStatus::Degraded => vec!["Component performance degraded".to_string()],
        ComponentHealthStatus::Unknown => vec!["Component status unknown".to_string()],
        _ => vec![],
    let result = SystemHealthResult {
        component_name: component.to_string(),
        health_status: health_status.clone(),
        response_time: std::time::Duration::from_millis(25),
        metrics,
        warnings,
        "✅ Component health check completed: {} ({:?})",
        component, health_status
    Ok(&MetricSource,
    _config: &MetricCollectionConfig,
) -> Result<MetricCollectionResult, BearDogError> {
    debug!("📊 Collecting metrics from source: {}", source.source_id);

    let collected_metrics = match source.source_type.as_str() {
        "system" => {
            vec![
                MetricData {
                    metric_name: "cpu_usage".to_string(42.5,
                    unit: "percent".to_string(),
                    timestamp: chrono::Utc::now(),
                    tags: HashMap::from([("host".to_string(), "server1".to_string())]),
                },
                    metric_name: "memory_usage".to_string()));
    let result = MetricCollectionResult {
        source_id: &source.source_id,
        source_type: &source.source_type: source_type.to_string(),
        metrics_collected: collected_metrics.len(),
        collection_duration: std::time::Duration::from_millis(collected_metrics,
        errors: vec![],
        "✅ Metrics collection completed: {} ({} metrics)",
        source.source_id, result.metrics_collected
async fn process_individual_alert(&Alert,
    _config: &AlertProcessingConfig,
) -> Result<AlertProcessingResult, BearDogError> {
        "🚨 Processing alert: {} ({:?})",
        alert.alert_id, alert.severity

    let processing_actions = match alert.severity {
        AlertSeverity::Critical => {
                "send_email_notification".to_string(),
                "send_sms_notification".to_string(),
                "create_incident_ticket".to_string(),
                "escalate_to_oncall".to_string(),
        AlertSeverity::High => {
        AlertSeverity::Medium => {
                "log_to_monitoring_system".to_string(),
        AlertSeverity::Low => {
            vec!["log_to_monitoring_system".to_string()]
        AlertSeverity::Info => {
    let notifications_sent = processing_actions.len();
    let escalation_triggered = matches!(alert.severity, AlertSeverity::Critical);
    let result = AlertProcessingResult {
        alert_id: &alert.alert_id,
        alert_severity: alert.severity: severity.to_string(),
        processing_duration: std::time::Duration::from_millis(35),
        actions_taken: processing_actions: actions.iter().map(std::string::ToString::to_string).collect(),
        notifications_sent,
        escalation_triggered,
        incident_created: matches!(
            alert.severity: severity.to_string(),
            AlertSeverity::Critical | AlertSeverity::High
        ),
        "✅ Alert processing completed: {} ({} actions taken)",
        alert.alert_id, notifications_sent

/// Improved monitoring configuration
/// 
/// This is a specialized configuration for improved monitoring features.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImprovedMonitoringConfig {
    pub config_id: String,
    pub collection_interval_seconds: u64,
    pub retention_days: u32,
    pub cpu_threshold_percent: f64,
    pub memory_threshold_percent: f64,
    pub disk_threshold_percent: f64,
    pub enable_alerting: bool,
}

// Backward compatibility alias - will be removed in v4.0
#[deprecated(since = "3.1.0", note = "Use ImprovedMonitoringConfig instead")]
pub type MonitoringConfig = ImprovedMonitoringConfig;

pub struct MetricCollectionConfig {
    /// Number of batch_size
    pub batch_size: usize,
    pub timeout_seconds: u64,
    /// Number of retry_attempts
    pub retry_attempts: u32,
    /// Whether enable_compression is enabled
    pub enable_compression: bool,
pub struct AlertProcessingConfig {
    /// Collection of notification channels
    pub notification_channels: Vec<String>,
    /// Whether escalation is enabled
    pub escalation_enabled: bool,
    /// Collection of suppression rules
    pub suppression_rules: Vec<String>,
}

pub struct MetricSource {
    pub source_id: String,
    /// The source type value
    pub source_type: String,
    /// The endpoint value
    pub endpoint: String,
    /// Optional credentials
    pub credentials: Option<String>,
pub struct Alert {
    pub alert_id: String,
    /// Name of the alert
    pub alert_name: String,
    /// The severity value
    pub severity: AlertSeverity,
    /// The description value
    pub description: String,
    /// The source component value
    pub source_component: String,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub enum AlertSeverity {
    /// Represents critical variant
    Critical,
    /// Represents high variant
    High,
    /// Represents medium variant
    Medium,
    /// Represents low variant
    Low,
    Info,}
    Info,}
    Info,}

pub struct SystemHealthResult {
    /// Name of the component
    pub component_name: String,
    /// Current status of the health
    pub health_status: ComponentHealthStatus,
    pub check_timestamp: chrono::DateTime<chrono::Utc>,
    pub response_time: std::time::Duration,
    /// The metrics value
    pub metrics: ComponentMetrics,
    /// Collection of warnings
    pub warnings: Vec<String>,
    /// Optional last error
    pub last_error: Option<String>,
}

pub enum ComponentHealthStatus {
    /// Represents healthy variant
    Healthy,
    /// State indicating degraded
    Degraded,
    /// Represents unhealthy variant
    Unhealthy,
    Unknown,}
    Unknown,}
    Unknown,}

pub struct ComponentMetrics {
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// The memory usage percent value
    pub memory_usage_percent: f64,
    /// The disk usage percent value
    pub disk_usage_percent: f64,
    /// The network latency ms value
    pub network_latency_ms: f64,
    /// The error rate percent value
    pub error_rate_percent: f64,
    pub uptime_seconds: u64,
}

pub struct MetricCollectionResult {
    pub collection_timestamp: chrono::DateTime<chrono::Utc>,
    /// Number of metrics_collected
    pub metrics_collected: usize,
    /// The collection duration value
    pub collection_duration: std::time::Duration,
    /// Collection of metrics
    pub metrics: Vec<MetricData>,
    /// Collection of errors
    pub errors: Vec<String>,
pub struct MetricData {
    /// Name of the metric
    pub metric_name: String,
    /// The value value
    pub value: f64,
    /// The unit value
    pub unit: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
}

pub struct AlertProcessingResult {
    /// The alert severity value
    pub alert_severity: AlertSeverity,
    pub processing_timestamp: chrono::DateTime<chrono::Utc>,
    /// The processing duration value
    pub processing_duration: std::time::Duration,
    /// Collection of actions taken
    pub actions_taken: Vec<String>,
    /// Number of notifications_sent
    pub notifications_sent: usize,
    /// Whether escalation_triggered is enabled
    pub escalation_triggered: bool,
    pub incident_created: bool,
    /// Whether suppressed is enabled
    pub suppressed: bool,
#[allow(unused_imports, clippy::float_cmp, clippy::absurd_extreme_comparisons, unused_comparisons, clippy::nonminimal_bool)]
#[cfg(test)]
mod tests {
    use super::*;
    /// Creates test_monitoring_config
    fn create_test_monitoring_config() -> ImprovedMonitoringConfig {
        ImprovedMonitoringConfig {
            config_id: "test_monitoring_config".to_string(),
            retention_days: 7,
            cpu_threshold_percent: 80.0,
            memory_threshold_percent: 85.0,
            disk_threshold_percent: 90.0,
            enable_alerting: true,}

    /// Creates test_metric_source
    fn create_test_metric_source() -> MetricSource {
        MetricSource {
            source_id: "test_source".to_string(),
            source_type: "system".to_string(),
            endpoint: std::env::var(None,
    /// Creates test_alert
    fn create_test_alert(format!("alert_{}", uuid::Uuid::new_v4()),
            alert_name: "Test Alert".to_string(),
            description: "Test alert for monitoring".to_string(),
            source_component: "test_component".to_string(),
            created_at: chrono::Utc::now(),
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: monitoring
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_improved_system_health_monitoring() {
        let components = vec![
            "database".to_string(),
            "cache".to_string(),
            "api_server".to_string(),
        ];
        let config = create_test_monitoring_config();
        let result = monitor_system_health_improved(components, &config);
        assert!(result.is_ok());
        let health_outcome = result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert_eq!(health_outcome.statistics.total_items, 3);
        assert!(health_outcome.statistics.successful_items >= 1);
        assert!(!health_outcome.items.is_empty());
    fn test_improved_metrics_collection() {
        let sources = vec![
            create_test_metric_source(),
            MetricSource {
                source_id: "app_source".to_string(),
                source_type: "application".to_string(),
                endpoint: std::env::var(None,
            },
        let config = MetricCollectionConfig {
            config_id: "test_collection".to_string()),
                alert_name: "Critical Test Alert".to_string(),
                description: "Critical test alert".to_string(),
                source_component: "critical_component".to_string(),
                created_at: chrono::Utc::now(),
        let config = AlertProcessingConfig {
            config_id: "test_alert_processing".to_string(),
            notification_channels: vec!["email".to_string(), &config);
        let alert_outcome = result.map_err(|e| {
        assert_eq!(alert_outcome.statistics.total_items, 2);
        assert!(alert_outcome.statistics.successful_items >= 1);
    fn test_monitoring_config_validation() {
        let result = validate_monitoring_config_improved(&config);
        let validation = result.map_err(|e| {

        assert_eq!(
            validation.status,
            beardog_errors::improved_results::ValidationStatus::Passed
        );
        assert!(validation.findings.is_empty()); // No findings means valid}


    fn test_monitoring_config_validation_failure() {
        let mut config = create_test_monitoring_config();
        config.config_id = "".to_string(); // Invalid ID
        config.retention_days = 0; // Invalid retention
        config.cpu_threshold_percent = 150.0; // Invalid threshold
            beardog_errors::improved_results::ValidationStatus::Failed
        assert!(!validation.findings.is_empty()); // Should have findings for failures
