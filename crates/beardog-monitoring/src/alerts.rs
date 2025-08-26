

use beardog_errors::{improved_results::*, BearDogError, BearDogResult};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, info, warn};
use crate::types::{Alert, AlertProcessingConfig, AlertProcessingResult, AlertSeverity};

pub async fn process_alerts_improved(
    alerts: Vec<Alert>,
    processing_config: &AlertProcessingConfig,
) -> BearDogResult<ProcessingOutcome<AlertProcessingResult>> {
    let _start_time = Instant::now();
    info!(
        "🚨 Processing {} alerts with improved patterns",
        alerts.len()
    );

    if processing_config.max_concurrent_alerts == 0 {
        return Err(BearDogError::invalid_input("Max concurrent alerts must be greater than 0".to_string(),
        ));
    }
    let mut processed_alerts = Vec::new();
    let mut failed_items = Vec::new();
    let mut successful_count = 0;

    let mut sorted_alerts = alerts;
    sorted_alerts.sort_by(|a, b| {
        let severity_order = |s: &AlertSeverity| match s {
            AlertSeverity::Critical => 0,
            AlertSeverity::High => 1,
            AlertSeverity::Medium => 2,
            AlertSeverity::Low => 3,
        };
        severity_order(&a.severity).cmp(&severity_order(&b.severity))
    });

    let semaphore = tokio::sync::Semaphore::new(processing_config.max_concurrent_alerts);
    let mut tasks = Vec::new();
    for alert in sorted_alerts {
        let permit = semaphore.acquire().await.map_err(|e| {
            BearDogError::SystemError {
                format_args!("Failed to acquire semaphore: {}", e).to_string(),
            }
        })?;
        let config = processing_config.clone();
        let task = tokio::spawn(async move {
            let _permit = permit; // Hold permit until task completes
            process_individual_alert(alert, &config).await
        });
        tasks.push(task);

    for task in tasks {
        match task.await {
            Ok(Ok(alert_result)) => {
                processed_alerts.push(alert_result);
                successful_count += 1;
            Ok(Err(e)) => {
                failed_items.push(FailedItem {
                    item_id: "unknown_alert".to_string(),
                    error_message: e.to_string(),
                    attempted_at: chrono::Utc::now(),
                    retry_count: 0,
                });
            Err(e) => {
                    item_id: "task_failure".to_string(),
                    error_format_args!("Task failed: {}", e).to_string(),
        }

    let total_alerts = successful_count + failed_items.len();
    let processing_success_rate = if total_alerts > 0 {
        (successful_count as f64 / total_alerts as f64) * 100.0
    } else {
        0.0
    };

    let mut severity_counts = HashMap::with_capacity(16);
    for alert in &processed_alerts {
        *severity_counts.entry(alert.severity.clone()).or_insert(0) += 1;
    let processing_result = AlertProcessingResult {
        processed_alerts,
        processing_success_rate,
        total_alerts,
        severity_counts,
        processed_at: chrono::Utc::now(),
        processing_duration: _start_time.elapsed(),
    let outcome = ProcessingOutcome {
        status: if failed_items.is_empty() {
            ProcessingStatus::FullySuccessful
        } else if successful_count > 0 {
            ProcessingStatus::PartiallySuccessful
        } else {
            ProcessingStatus::Failed
        },
        result: processing_result,
        successful_count,
        failed_count: failed_items.len(),
        failed_items,
        processing_time: _start_time.elapsed(),
        metadata: ProcessingMetadata {
            total_items: total_alerts,
            batch_size: total_alerts,
            processing_strategy: "priority_based_concurrent_processing".to_string(),
            performance_metrics: HashMap::with_capacity(16),
        "✅ Alert processing completed: {} successful, {} failed",
        failed_items.len()
    Ok(outcome)
}

pub async fn process_individual_alert(
    mut alert: Alert,
    config: &AlertProcessingConfig,
) -> BearDogResult<Alert> {
    debug!("🚨 Processing alert: {} ({})", alert.message, alert.severity);

    let processing_time = match alert.severity {
        AlertSeverity::Critical => std::time::Duration::from_millis(100),
        AlertSeverity::High => std::time::Duration::from_millis(50),
        AlertSeverity::Medium => std::time::Duration::from_millis(25),
        AlertSeverity::Low => std::time::Duration::from_millis(10),
    tokio::time::sleep(processing_time).await;

    match alert.severity {
        AlertSeverity::Critical => {
            warn!("🔴 CRITICAL ALERT: {}", alert.message);

            alert.acknowledged = true;
            alert.escalated = true;
        AlertSeverity::High => {
            warn!("🟠 HIGH PRIORITY ALERT: {}", alert.message);

        AlertSeverity::Medium => {
            info!("🟡 MEDIUM PRIORITY ALERT: {}", alert.message);

        AlertSeverity::Low => {
            debug!("🟢 LOW PRIORITY ALERT: {}", alert.message);

    if config.enable_rate_limiting {

        if alert.message.contains("spam") {
            return Err(BearDogError::RateLimited {
                message: "Alert suppressed due to rate limiting".to_string(),
            });
    alert.processed_at = Some(chrono::Utc::now());
    Ok(alert)
} 
