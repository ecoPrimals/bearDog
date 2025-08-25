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


/// Metrics Collection
///
/// This module provides improved metrics collection using ProcessingOutcome patterns.

use beardog_errors::{improved_results::*, BearDogError, BearDogResult};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, info};
use crate::types::{MetricCollectionConfig, MetricSource, MetricCollectionResult, MetricValue};
/// Improved metrics collection using ProcessingOutcome
pub async fn collect_metrics_improved(
    sources: Vec<MetricSource>,
    collection_config: &MetricCollectionConfig,
) -> BearDogResult<ProcessingOutcome<MetricCollectionResult>> {
    let _start_time = Instant::now();
    info!(
        "📊 Collecting metrics from {} sources with improved patterns",
        sources.len()
    );
    // Validate collection configuration
    if collection_config.collection_interval < 1 {
        return Err(BearDogError::invalid_input("Collection interval must be at least 1 second".to_string(),
        ));
    }
    let mut collected_metrics = HashMap::new();
    let mut failed_items = Vec::new();
    let mut successful_count = 0;
    // Collect metrics from each source
    for source in sources {
        match collect_source_metrics(&source, collection_config).await {
            Ok(metrics) => {
                for (key, value) in metrics {
                    collected_metrics.insert(format!("{}_{}", source.name, key), value);
                }
                successful_count += 1;
            }
            Err(e) => {
                debug!("❌ Failed to collect metrics from {}: {}", source.name, e);
                failed_items.push(FailedItem {
                    item_id: source.name.clone(),
                    error_message: e.to_string(),
                    attempted_at: chrono::Utc::now(),
                    retry_count: 0,
                });
        }
    // Calculate collection statistics
    let total_sources = successful_count + failed_items.len();
    let collection_success_rate = if total_sources > 0 {
        (successful_count as f64 / total_sources as f64) * 100.0
    } else {
        0.0
    };
    let collection_result = MetricCollectionResult {
        metrics: collected_metrics,
        collection_success_rate,
        sources_count: total_sources,
        collected_at: chrono::Utc::now(),
        next_collection_in: std::time::Duration::from_secs(collection_config.collection_interval),
    let outcome = ProcessingOutcome {
        status: if failed_items.is_empty() {
            ProcessingStatus::FullySuccessful
        } else if successful_count > 0 {
            ProcessingStatus::PartiallySuccessful
        } else {
            ProcessingStatus::Failed
        },
        result: collection_result,
        successful_count,
        failed_count: failed_items.len(),
        failed_items,
        processing_time: _start_time.elapsed(),
        metadata: ProcessingMetadata {
            total_items: total_sources,
            batch_size: total_sources,
            processing_strategy: "parallel_metric_collection".to_string(),
            performance_metrics: HashMap::new(),
        "✅ Metrics collection completed: {} successful, {} failed",
        failed_items.len()
    Ok(outcome)
}
/// Collect metrics from a specific source
pub async fn collect_source_metrics(
    source: &MetricSource,
    _config: &MetricCollectionConfig,
) -> BearDogResult<HashMap<String, MetricValue>> {
    debug!("📊 Collecting metrics from source: {}", source.name);
    // Simulate metrics collection with realistic timing
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    let mut metrics = HashMap::new();
    // Generate realistic metrics based on source type
    match source.source_type.as_str() {
        "system" => {
            metrics.insert("cpu_usage".to_string(), MetricValue::Percentage(45.2));
            metrics.insert("memory_usage".to_string(), MetricValue::Percentage(67.8));
            metrics.insert("disk_usage".to_string(), MetricValue::Percentage(23.1));
            metrics.insert("network_throughput".to_string(), MetricValue::Rate(1024.5));
        "application" => {
            metrics.insert("request_count".to_string(), MetricValue::Counter(15423));
            metrics.insert("response_time".to_string(), MetricValue::Duration(125.3));
            metrics.insert("error_rate".to_string(), MetricValue::Percentage(0.12));
            metrics.insert("active_connections".to_string(), MetricValue::Gauge(234));
        "security" => {
            metrics.insert("failed_logins".to_string(), MetricValue::Counter(3));
            metrics.insert("active_sessions".to_string(), MetricValue::Gauge(567));
            metrics.insert("threat_level".to_string(), MetricValue::Level("low".to_string()));
            metrics.insert("encryption_operations".to_string(), MetricValue::Counter(8934));
        _ => {
            metrics.insert("generic_metric".to_string(), MetricValue::Gauge(100));
    Ok(metrics)
} 
