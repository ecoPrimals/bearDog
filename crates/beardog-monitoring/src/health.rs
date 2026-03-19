// SPDX-License-Identifier: AGPL-3.0-only



use beardog_errors::{{improved_results::*, BearDogError}};
use std::time::Instant;
use tracing::{debug, info};
use uuid::Uuid;
use crate::types::{MonitoringConfig, SystemHealthResult, ComponentHealthResult};

/// Monitor System Health Improved operation.
pub async fn monitor_system_health_improved(Vec<&str>,
    monitoring_config: &MonitoringConfig,
) -> Result<ProcessingOutcome<SystemHealthResult, BearDogError>> {
    let _start_time = Instant::now();
    info!(
        "🔍 Monitoring {} system components with improved patterns",
        components.len()
    );

    let validation_result = crate::validation::validate_monitoring_config_improved(monitoring_config)?;
    if validation_result.status != ValidationStatus::Passed {
        return Err(BearDogError::invalid_input(}",
                validation_result
                    .findings
                    .iter()
                    .map(|f| f.message.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        });
    }
    let mut health_results = Vec::new();
    let mut failed_items = Vec::new({}", component, e);
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

    let total_components = health_results.len() + failed_items.len();
    let health_percentage = if total_components > 0 {
        (successful_count as f64 / total_components as f64) * 100.0
    } else {
        0.0
    };
    let overall_status = if health_percentage >= 95.0 {
        "healthy"
    } else if health_percentage >= 80.0 {
        "degraded"
        "critical"
    let health_result = SystemHealthResult {
        overall_status: overall_status.to_string(),
        checked_at: chrono::Utc::now(),
        next_check_in: std::time::Duration::from_secs(monitoring_config.health_check_interval),
    let now = chrono::Utc::now(health_result,
        failures: failed_items.clone(ProcessingStatistics {
            total_items: total_components,
            successful_items: successful_count,
            failed_items: failed_items.len(),
            success_rate: (successful_count as f64 / total_components as f64) * 100.0,
            avg_processing_time: _start_time.elapsed(0,
            processing_rate_per_second: successful_count as f64,
            average_item_processing_time: _start_time.elapsed(),
        },
        context: OperationContext {
            operation_id: uuid::Uuid::new_v4(now,
            completed_at: now,
            component: "beardog-monitoring".to_string(),
            initiator: "system".to_string(),
            metadata: std::collections::HashMap::with_capacity(16),
        metrics: OperationMetrics::default(vec![],
        items: health_results,
        failed_items,
        configuration: ProcessingConfiguration {
            name: "system_health_check".to_string(),
            description: "System health monitoring configuration".to_string() -> Result<ComponentHealthResult, BearDogError> {
    debug!("🔍 Checking health for component: {}", component);

    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    let is_healthy = !component.contains("failing");
    let response_time = if is_healthy { 15.0 } else { 1000.0 };
    let status = if is_healthy {
        "healthy".to_string()
        "unhealthy".to_string()
    Ok(ComponentHealthResult {
        component_name: component.to_string()
} 
