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


/// System Health Monitoring
///
/// This module provides improved system health monitoring using ProcessingOutcome patterns.

use beardog_errors::{improved_results::*, BearDogError, BearDogResult};
use std::time::Instant;
use tracing::{debug, info};
use uuid::Uuid;
use crate::types::{MonitoringConfig, SystemHealthResult, ComponentHealthResult};
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
    let validation_result = crate::validation::validate_monitoring_config_improved(monitoring_config).await?;
    if validation_result.status != ValidationStatus::Passed {
        return Err(BearDogError::invalid_input(format!(
                "Monitoring configuration validation failed: }",
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
                debug!("❌ Component {} health check failed: {}", component, e);
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
    // Calculate overall health metrics
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
        component_results: health_results,
        health_percentage,
        checked_at: chrono::Utc::now(),
        next_check_in: std::time::Duration::from_secs(monitoring_config.health_check_interval),
    let now = chrono::Utc::now();
    let outcome = ProcessingOutcome {
        result: health_result,
        failures: failed_items.clone(),
        statistics: ProcessingStatistics {
            total_items: total_components,
            successful_items: successful_count,
            failed_items: failed_items.len(),
            success_rate: (successful_count as f64 / total_components as f64) * 100.0,
            avg_processing_time: _start_time.elapsed(),
            skipped_items: 0,
            processing_rate_per_second: successful_count as f64,
            average_item_processing_time: _start_time.elapsed(),
        },
        context: OperationContext {
            operation_id: uuid::Uuid::new_v4().to_string(),
            started_at: now,
            completed_at: now,
            component: "beardog-monitoring".to_string(),
            initiator: "system".to_string(),
            request_id: None,
            metadata: std::collections::HashMap::new(),
        metrics: OperationMetrics::default(),
        warnings: vec![],
        items: health_results,
        failed_items,
        configuration: ProcessingConfiguration {
            name: "system_health_check".to_string(),
            description: "System health monitoring configuration".to_string(),
            batch_size: total_components,
            max_retries: 2,
            timeout_seconds: 30,
            parallel_processing: true,
            retry_delay_seconds: 1,
            enable_detailed_logging: true,
        "✅ Health monitoring completed: {} successful, {} failed",
        successful_count,
        failed_items.len()
    Ok(outcome)
}
/// Monitor individual component health
pub async fn monitor_component_health(
    component: &str,
    config: &MonitoringConfig,
) -> BearDogResult<ComponentHealthResult> {
    debug!("🔍 Checking health for component: {}", component);
    // Simulate component health check with realistic timing
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    // In a real implementation, this would check actual component status
    let is_healthy = !component.contains("failing");
    let response_time = if is_healthy { 15.0 } else { 1000.0 };
    let status = if is_healthy {
        "healthy".to_string()
        "unhealthy".to_string()
    Ok(ComponentHealthResult {
        component_name: component.to_string(),
        status,
        response_time_ms: response_time,
        last_error: if is_healthy { None } else { Some("Component is marked as failing".to_string()) },
        uptime_percentage: if is_healthy { 99.9 } else { 75.0 },
        dependencies_healthy: is_healthy,
    })
} 
