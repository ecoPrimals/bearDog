// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Improved Monitoring Operations with Rich Context
///
/// This module demonstrates the new idiomatic monitoring patterns using
/// ProcessingOutcome and specialized outcome types instead of traditional patterns.

use beardog_errors::{improved_results::*, BearDogError, BearDogResult};
use serde_json;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, info};
/// Improved system health monitoring using ProcessingOutcome
pub async fn monitor_system_health_improved(
    components: Vec<String>,
    monitoring_config: &MonitoringConfig,
) -> BearDogResult<ProcessingOutcome<SystemHealthResult>> {
    let _start_time = Instant::now();
    info!(
        "🔍 Monitoring {} system components with improved patterns",
        components.len()
    );
    // Validate monitoring configuration
    let validation_result = validate_monitoring_config_improved(monitoring_config).await?;
    if validation_result.status != ValidationStatus::Passed {
        return Err(BearDogError::invalid_input(format!(
            "Monitoring configuration validation failed: {}",
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
    // Monitor each component
    for component in components {
        match monitor_component_health(&component, monitoring_config).await {
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
                    attempted_at: now,
                    item_id: component,
                    error_code: "HEALTH_CHECK_FAILED".to_string(),
                    error_message: e.to_string(),
                    retry_count: 0,
                    failed_at: now,
                });
        }
    let total_items = health_results.len() as u64 + failed_items.len() as u64;
    let processing_rate = successful_count as f64; // Simplified rate calculation
        "✅ System health monitoring completed: {}/{} components healthy",
        successful_count, total_items
    let now = chrono::Utc::now();
    // Create a summary result instead of the full Vec
    let summary_result = SystemHealthResult {
        component_name: "system_health_summary".to_string(),
        health_status: if failed_items.is_empty() {
            ComponentHealthStatus::Healthy
        } else {
            ComponentHealthStatus::Degraded
        },
        check_timestamp: chrono::Utc::now(),
        response_time: std::time::Duration::from_millis(100),
        metrics: ComponentMetrics {
            cpu_usage_percent: 0.0,
            memory_usage_percent: 0.0,
            disk_usage_percent: 0.0,
            network_latency_ms: 0.0,
            error_rate_percent: 0.0,
            uptime_seconds: 0,
        warnings: vec![],
        last_error: None,
    };
    Ok(ProcessingOutcome {
        result: summary_result,
        failures: failed_items.clone(),
        statistics: ProcessingStatistics {
            total_items: total_items as usize,
            successful_items: successful_count,
            failed_items: failed_items.len(),
            success_rate: (successful_count as f64 / total_items as f64) * 100.0,
            avg_processing_time: std::time::Duration::from_millis(100),
            skipped_items: 0,
            processing_rate_per_second: processing_rate,
            average_item_processing_time: std::time::Duration::from_millis(100),
        context: OperationContext {
            operation_id: uuid::Uuid::new_v4().to_string(),
            started_at: now,
            completed_at: now,
            component: "beardog-monitoring".to_string(),
            initiator: "system".to_string(),
            request_id: None,
            metadata: std::collections::HashMap::new(),
        metrics: OperationMetrics::default(),
        items: health_results,
        failed_items,
        configuration: ProcessingConfiguration {
            name: "system_health_check".to_string(),
            description: "System health monitoring configuration".to_string(),
            batch_size: 5,
            max_retries: 2,
            timeout_seconds: 30,
            parallel_processing: true,
    })
}
/// Improved metrics collection using ProcessingOutcome
pub async fn collect_metrics_improved(
    metric_sources: Vec<MetricSource>,
    collection_config: &MetricCollectionConfig,
) -> BearDogResult<ProcessingOutcome<MetricCollectionResult>> {
    let total_sources = metric_sources.len() as u64;
        "📊 Collecting metrics from {} sources with improved patterns",
        total_sources
    let mut collection_results = Vec::new();
    for source in metric_sources {
        match collect_source_metrics(&source, collection_config).await {
            Ok(result) => {
                collection_results.push(result);
                    item_value: serde_json::json!(source.source_id),
                    item_id: source.source_id,
                    error_code: "METRIC_COLLECTION_FAILED".to_string(),
        "✅ Metrics collection completed: {}/{} sources successful",
        successful_count, total_sources
    let summary_result = MetricCollectionResult {
        source_id: "metrics_collection_summary".to_string(),
        source_type: "summary".to_string(),
        collection_timestamp: chrono::Utc::now(),
        metrics_collected: successful_count,
        collection_duration: std::time::Duration::from_millis(80),
        metrics: vec![],
        errors: if failed_items.is_empty() {
            vec![]
            vec![format!("{} sources failed", failed_items.len())]
            total_items: total_sources as usize,
            success_rate: (successful_count as f64 / total_sources as f64) * 100.0,
            avg_processing_time: std::time::Duration::from_millis(80),
            average_item_processing_time: std::time::Duration::from_millis(80),
        items: collection_results,
            name: "metrics_collection".to_string(),
            description: "Metrics collection configuration".to_string(),
            batch_size: 10,
            max_retries: 3,
            timeout_seconds: 15,
/// Improved alert processing using ProcessingOutcome
pub async fn process_alerts_improved(
    alerts: Vec<Alert>,
    alert_config: &AlertProcessingConfig,
) -> BearDogResult<ProcessingOutcome<AlertProcessingResult>> {
    let total_alerts = alerts.len() as u64;
        "🚨 Processing {} alerts with improved patterns",
        total_alerts
    let mut processing_results = Vec::new();
    for alert in alerts {
        match process_individual_alert(&alert, alert_config).await {
                processing_results.push(result);
                    item_value: serde_json::json!(alert.alert_id),
                    item_id: alert.alert_id,
                    error_code: "ALERT_PROCESSING_FAILED".to_string(),
        "✅ Alert processing completed: {}/{} alerts processed",
        successful_count, total_alerts
    let summary_result = AlertProcessingResult {
        alert_id: "alert_processing_summary".to_string(),
        alert_severity: AlertSeverity::Info,
        processing_timestamp: chrono::Utc::now(),
        processing_duration: std::time::Duration::from_millis(60),
        actions_taken: vec![format!(
            "{}/{} alerts processed successfully",
            successful_count, total_alerts
        )],
        notifications_sent: successful_count,
        escalation_triggered: false,
        incident_created: false,
        suppressed: false,
            total_items: total_alerts as usize,
            success_rate: (successful_count as f64 / total_alerts as f64) * 100.0,
            avg_processing_time: std::time::Duration::from_millis(60),
            average_item_processing_time: std::time::Duration::from_millis(60),
        items: processing_results,
            name: "alert_processing".to_string(),
            description: "Alert processing configuration".to_string(),
            batch_size: 20,
            max_retries: 1,
            timeout_seconds: 10,
/// Improved monitoring configuration validation
async fn validate_monitoring_config_improved(
    config: &MonitoringConfig,
) -> BearDogResult<ValidationOutcome> {
    debug!(
        "🔍 Validating monitoring configuration: {}",
        config.config_id
    let mut findings = Vec::new();
    let mut score: f64 = 1.0;
    // Validate configuration ID
    if config.config_id.is_empty() {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            field: "config.config_id".to_string(),
            message: "Monitoring configuration ID cannot be empty".to_string(),
            config_path: Some("config.config_id".to_string()),
            suggested_fix: Some("Provide a valid configuration ID".to_string()),
            suggestion: Some("Provide a valid configuration ID".to_string()),
            code: "MON001".to_string(),
        });
        score -= 0.5;
    // Validate collection interval
    if config.collection_interval_seconds < 5 {
            severity: FindingSeverity::Warning,
            field: "config.collection_interval_seconds".to_string(),
            message: "Collection interval is very short - may impact performance".to_string(),
            config_path: Some("config.collection_interval_seconds".to_string()),
            suggested_fix: Some(
                "Consider increasing collection interval to at least 5 seconds".to_string(),
            ),
            suggestion: Some(
            code: "MON002".to_string(),
        score -= 0.1;
    // Validate retention period
    if config.retention_days < 1 {
            severity: FindingSeverity::Error,
            field: "config.retention_days".to_string(),
            message: "Retention period must be at least 1 day".to_string(),
            config_path: Some("config.retention_days".to_string()),
            suggested_fix: Some("Set retention period to at least 1 day".to_string()),
            suggestion: Some("Set retention period to at least 1 day".to_string()),
            code: "MON003".to_string(),
        score -= 0.3;
    // Validate alert thresholds
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
            required: true,
            weight: 0.4,
            passed: !findings
                .any(|f| f.field.starts_with("config") && f.severity == FindingSeverity::Critical),
            name: "Collection Settings".to_string(),
            description: "Metric collection interval and retention settings".to_string(),
            weight: 0.3,
            passed: config.collection_interval_seconds >= 5 && config.retention_days >= 1,
            name: "Alert Thresholds".to_string(),
            description: "Alert threshold configuration and limits".to_string(),
            required: false,
            passed: config.cpu_threshold_percent >= 0.0 && config.cpu_threshold_percent <= 100.0,
    ];
        "✅ Monitoring configuration validation completed: {} (Score: {:.2})",
        config.config_id, score
    Ok(ValidationOutcome {
        findings,
        status: if valid {
            ValidationStatus::Passed
            ValidationStatus::Failed
        criteria,
            started_at: chrono::Utc::now(),
            completed_at: chrono::Utc::now(),
/// Helper functions for monitoring operations
async fn monitor_component_health(
    component: &str,
    _config: &MonitoringConfig,
) -> BearDogResult<SystemHealthResult> {
    debug!("🔍 Monitoring component health: {}", component);
    // Simulate component health monitoring
    let health_status = match component {
        "database" => {
            // Simulate database health check
        "cache" => {
            // Simulate cache health check
        "api_server" => {
            // Simulate API server health check
        "hsm" => {
            // Simulate HSM health check
        _ => {
            // Unknown component
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
    Ok(result)
async fn collect_source_metrics(
    source: &MetricSource,
    _config: &MetricCollectionConfig,
) -> BearDogResult<MetricCollectionResult> {
    debug!("📊 Collecting metrics from source: {}", source.source_id);
    // Simulate metric collection based on source type
    let collected_metrics = match source.source_type.as_str() {
        "system" => {
            vec![
                MetricData {
                    metric_name: "cpu_usage".to_string(),
                    value: 42.5,
                    unit: "percent".to_string(),
                    timestamp: chrono::Utc::now(),
                    tags: HashMap::from([("host".to_string(), "server1".to_string())]),
                },
                    metric_name: "memory_usage".to_string(),
                    value: 68.2,
            ]
        "application" => {
                    metric_name: "request_count".to_string(),
                    value: 1250.0,
                    unit: "count".to_string(),
                    tags: HashMap::from([("service".to_string(), "api".to_string())]),
                    metric_name: "response_time".to_string(),
                    value: 125.5,
                    unit: "milliseconds".to_string(),
        "security" => {
                    metric_name: "failed_logins".to_string(),
                    value: 3.0,
                    tags: HashMap::from([("type".to_string(), "security".to_string())]),
                    metric_name: "threat_detections".to_string(),
                    value: 0.0,
            return Err(BearDogError::invalid_input(format!(
                "Unknown metric source type: {}",
                source.source_type
            )));
    let result = MetricCollectionResult {
        source_id: source.source_id.clone(),
        source_type: source.source_type.clone(),
        metrics_collected: collected_metrics.len(),
        collection_duration: std::time::Duration::from_millis(45),
        metrics: collected_metrics,
        errors: vec![],
        "✅ Metrics collection completed: {} ({} metrics)",
        source.source_id, result.metrics_collected
async fn process_individual_alert(
    alert: &Alert,
    _config: &AlertProcessingConfig,
) -> BearDogResult<AlertProcessingResult> {
        "🚨 Processing alert: {} ({:?})",
        alert.alert_id, alert.severity
    // Simulate alert processing based on severity
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
        alert_id: alert.alert_id.clone(),
        alert_severity: alert.severity.clone(),
        processing_duration: std::time::Duration::from_millis(35),
        actions_taken: processing_actions,
        notifications_sent,
        escalation_triggered,
        incident_created: matches!(
            alert.severity,
            AlertSeverity::Critical | AlertSeverity::High
        ),
        "✅ Alert processing completed: {} ({} actions taken)",
        alert.alert_id, notifications_sent
/// Custom types for monitoring operations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MonitoringConfig {
    pub config_id: String,
    pub collection_interval_seconds: u64,
    pub retention_days: u32,
    pub cpu_threshold_percent: f64,
    pub memory_threshold_percent: f64,
    pub disk_threshold_percent: f64,
    pub enable_alerting: bool,
}


pub struct MetricCollectionConfig {
    pub batch_size: usize,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub enable_compression: bool,
pub struct AlertProcessingConfig {
    pub notification_channels: Vec<String>,
    pub escalation_enabled: bool,
    pub suppression_rules: Vec<String>,
}


pub struct MetricSource {
    pub source_id: String,
    pub source_type: String,
    pub endpoint: String,
    pub credentials: Option<String>,
pub struct Alert {
    pub alert_id: String,
    pub alert_name: String,
    pub severity: AlertSeverity,
    pub description: String,
    pub source_component: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}


pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,}


pub struct SystemHealthResult {
    pub component_name: String,
    pub health_status: ComponentHealthStatus,
    pub check_timestamp: chrono::DateTime<chrono::Utc>,
    pub response_time: std::time::Duration,
    pub metrics: ComponentMetrics,
    pub warnings: Vec<String>,
    pub last_error: Option<String>,
}


pub enum ComponentHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,}


pub struct ComponentMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub network_latency_ms: f64,
    pub error_rate_percent: f64,
    pub uptime_seconds: u64,
}


pub struct MetricCollectionResult {
    pub collection_timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics_collected: usize,
    pub collection_duration: std::time::Duration,
    pub metrics: Vec<MetricData>,
    pub errors: Vec<String>,
pub struct MetricData {
    pub metric_name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub tags: HashMap<String, String>,
}


pub struct AlertProcessingResult {
    pub alert_severity: AlertSeverity,
    pub processing_timestamp: chrono::DateTime<chrono::Utc>,
    pub processing_duration: std::time::Duration,
    pub actions_taken: Vec<String>,
    pub notifications_sent: usize,
    pub escalation_triggered: bool,
    pub incident_created: bool,
    pub suppressed: bool,
#[cfg(test)]
mod tests {
    use super::*;
    fn create_test_monitoring_config() -> MonitoringConfig {
        MonitoringConfig {
            config_id: "test_monitoring_config".to_string(),
            collection_interval_seconds: 30,
            retention_days: 7,
            cpu_threshold_percent: 80.0,
            memory_threshold_percent: 85.0,
            disk_threshold_percent: 90.0,
            enable_alerting: true,}


    fn create_test_metric_source() -> MetricSource {
        MetricSource {
            source_id: "test_source".to_string(),
            source_type: "system".to_string(),
            endpoint: "http://localhost:9090".to_string(),
            credentials: None,
    fn create_test_alert() -> Alert {
        Alert {
            alert_id: format!("alert_{}", uuid::Uuid::new_v4()),
            alert_name: "Test Alert".to_string(),
            severity: AlertSeverity::Medium,
            description: "Test alert for monitoring".to_string(),
            source_component: "test_component".to_string(),
            created_at: chrono::Utc::now(),
    #[tokio::test]
    async fn test_improved_system_health_monitoring() {
        let components = vec![
            "database".to_string(),
            "cache".to_string(),
            "api_server".to_string(),
        ];
        let config = create_test_monitoring_config();
        let result = monitor_system_health_improved(components, &config).await;
        assert!(result.is_ok());
        let health_outcome = result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert_eq!(health_outcome.statistics.total_items, 3);
        assert!(health_outcome.statistics.successful_items >= 1);
        assert!(!health_outcome.items.is_empty());
    async fn test_improved_metrics_collection() {
        let sources = vec![
            create_test_metric_source(),
            MetricSource {
                source_id: "app_source".to_string(),
                source_type: "application".to_string(),
                endpoint: "http://localhost:8080".to_string(),
                credentials: None,
            },
        let config = MetricCollectionConfig {
            config_id: "test_collection".to_string(),
            batch_size: 100,
            retry_attempts: 3,
            enable_compression: true,
        };
        let result = collect_metrics_improved(sources, &config).await;
        let metrics_outcome = result.map_err(|e| {
        assert_eq!(metrics_outcome.statistics.total_items, 2);
        assert!(metrics_outcome.statistics.successful_items >= 1);
    async fn test_improved_alert_processing() {
        let alerts = vec![
            create_test_alert(),
            Alert {
                alert_id: format!("critical_alert_{}", uuid::Uuid::new_v4()),
                alert_name: "Critical Test Alert".to_string(),
                severity: AlertSeverity::Critical,
                description: "Critical test alert".to_string(),
                source_component: "critical_component".to_string(),
                created_at: chrono::Utc::now(),
        let config = AlertProcessingConfig {
            config_id: "test_alert_processing".to_string(),
            notification_channels: vec!["email".to_string(), "sms".to_string()],
            escalation_enabled: true,
            suppression_rules: vec![],
        let result = process_alerts_improved(alerts, &config).await;
        let alert_outcome = result.map_err(|e| {
        assert_eq!(alert_outcome.statistics.total_items, 2);
        assert!(alert_outcome.statistics.successful_items >= 1);
    async fn test_monitoring_config_validation() {
        let result = validate_monitoring_config_improved(&config).await;
        let validation = result.map_err(|e| {
        // Use correct ValidationOutcome fields
        assert_eq!(
            validation.status,
            beardog_errors::improved_results::ValidationStatus::Passed
        );
        assert!(validation.findings.is_empty()); // No findings means valid}


    async fn test_monitoring_config_validation_failure() {
        let mut config = create_test_monitoring_config();
        config.config_id = "".to_string(); // Invalid ID
        config.retention_days = 0; // Invalid retention
        config.cpu_threshold_percent = 150.0; // Invalid threshold
            beardog_errors::improved_results::ValidationStatus::Failed
        assert!(!validation.findings.is_empty()); // Should have findings for failures
