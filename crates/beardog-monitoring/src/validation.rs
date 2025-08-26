

use beardog_errors::{improved_results::*, BearDogResult};
use tracing::debug;
use crate::types::MonitoringConfig;

pub async fn validate_monitoring_config_improved(
    config: &MonitoringConfig,
) -> BearDogResult<ValidationOutcome> {
    debug!("🔍 Validating monitoring configuration: {}", config.config_id);
    let mut findings = Vec::new();

    if config.config_id.is_empty() {
        findings.push(ValidationFinding {
            field: "config_id".to_string(),
            message: "Configuration ID cannot be empty".to_string(),
            severity: ValidationSeverity::Error,
            suggested_fix: Some("Provide a non-empty configuration ID".to_string()),
        });
    }

    if config.collection_interval_seconds == 0 {
            field: "collection_interval_seconds".to_string(),
            message: "Collection interval must be greater than 0".to_string(),
            suggested_fix: Some("Set collection interval to at least 1 second".to_string()),
    } else if config.collection_interval_seconds < 10 {
            message: "Collection interval below 10 seconds may impact performance".to_string(),
            severity: ValidationSeverity::Warning,
            suggested_fix: Some("Consider using intervals of 10 seconds or more".to_string()),

    if config.retention_days == 0 {
            field: "retention_days".to_string(),
            message: "Retention days must be greater than 0".to_string(),
            suggested_fix: Some("Set retention to at least 1 day".to_string()),
    } else if config.retention_days > 365 {
            message: "Retention period over 1 year may consume excessive storage".to_string(),
            suggested_fix: Some("Consider reducing retention to 365 days or less".to_string()),

    if !(0.0..=100.0).contains(&config.cpu_threshold_percent) {
            field: "cpu_threshold_percent".to_string(),
            message: "CPU threshold must be between 0 and 100".to_string(),
            suggested_fix: Some("Set CPU threshold between 0.0 and 100.0".to_string()),
    } else if config.cpu_threshold_percent < 50.0 {
            message: "CPU threshold below 50% may trigger excessive alerts".to_string(),
            suggested_fix: Some("Consider setting CPU threshold to 50% or higher".to_string()),

    if !(0.0..=100.0).contains(&config.memory_threshold_percent) {
            field: "memory_threshold_percent".to_string(),
            message: "Memory threshold must be between 0 and 100".to_string(),
            suggested_fix: Some("Set memory threshold between 0.0 and 100.0".to_string()),

    if !(0.0..=100.0).contains(&config.disk_threshold_percent) {
            field: "disk_threshold_percent".to_string(),
            message: "Disk threshold must be between 0 and 100".to_string(),
            suggested_fix: Some("Set disk threshold between 0.0 and 100.0".to_string()),

    if config.health_check_interval == 0 {
            field: "health_check_interval".to_string(),
            message: "Health check interval must be greater than 0".to_string(),
            suggested_fix: Some("Set health check interval to at least 1 second".to_string()),

    let has_errors = findings.iter().any(|f| matches!(f.severity, ValidationSeverity::Error));
    let status = if has_errors {
        ValidationStatus::Failed
    } else {
        ValidationStatus::Passed
    };
    let outcome = ValidationOutcome {
        status,
        findings,
        validated_at: chrono::Utc::now(),
        validation_duration: std::time::Duration::from_millis(1), // Simulated validation time
    debug!(
        "✅ Configuration validation completed: {} ({})",
        outcome.status, outcome.findings.len()
    );
    Ok(outcome)
} 
