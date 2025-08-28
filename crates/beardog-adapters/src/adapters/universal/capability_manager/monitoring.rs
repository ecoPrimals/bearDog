

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;
use super::super::traits::*;
use super::config::CapabilityManagerConfig;
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
pub struct CapabilityMonitor {

    pub capability_id: String,

    pub provider_key: String,

    pub last_health_check: DateTime<Utc>,

    pub current_performance: PerformanceMetrics,

    pub availability_history: Vec<AvailabilitySnapshot>,

    pub alert_thresholds: AlertThresholds,

    pub status: CapabilityStatus,
}

pub struct PerformanceMetrics {

    pub response_time_ms: u64,

    pub throughput_per_sec: u64,

    pub error_rate_percent: f64,

    pub resource_utilization: ResourceUtilization,

    pub quality_score: f64,

pub struct ResourceUtilization {

    pub cpu_percent: f64,

    pub memory_percent: f64,

    pub network_mbps: f64,

    pub storage_iops: u64,

pub struct AvailabilitySnapshot {

    pub timestamp: DateTime<Utc>,

    pub available: bool,

pub struct AlertThresholds {

    pub max_response_time_ms: u64,

    pub min_availability_percent: f64,

    pub max_error_rate_percent: f64,

    pub min_quality_score: f64,

pub enum CapabilityStatus {

    Healthy,

    Degraded,

    Critical,

    Offline,

    Unknown,}

impl CapabilityMonitor {

    pub async fn update_capability_monitor(
        monitors: &Arc<RwLock<HashMap<&str, CapabilityMonitor>>>,
        provider_key: &str,
        capability: &Capability,
        config: &CapabilityManagerConfig,
    ) -> Result<(), BearDogError> {
        let mut monitors_guard = monitors.write().await;
        let monitor_key = format_args!("{}:{}", provider_key, capability.id).to_string();

        let monitor = monitors_guard
            .entry(monitor_key.clone())
            .or_insert_with(|| CapabilityMonitor {
                capability_id: capability.id.clone(),
                provider_key: provider_key.to_string(),
                last_health_check: Utc::now(),
                current_performance: PerformanceMetrics {
                    response_time_ms: 0,
                    throughput_per_sec: 0,
                    error_rate_percent: 0.0,
                    resource_utilization: ResourceUtilization {
                        cpu_percent: 0.0,
                        memory_percent: 0.0,
                        network_mbps: 0.0,
                        storage_iops: 0,
                    },
                    quality_score: 1.0,
                },
                availability_history: Vec::new(),
                alert_thresholds: AlertThresholds {
                    max_response_time_ms: 1000,
                    min_availability_percent: 95.0,
                    max_error_rate_percent: 5.0,
                    min_quality_score: 0.8,
                status: CapabilityStatus::Unknown,
            });

        let new_performance = PerformanceMetrics {
            response_time_ms: 50 + (capability.id.len() as u64 * 10) % 200,
            throughput_per_sec: 100 + (capability.id.len() as u64 * 20) % 500,
            error_rate_percent: (capability.id.len() as f64 * 0.1) % 2.0,
            resource_utilization: ResourceUtilization {
                cpu_percent: (capability.id.len() as f64 * 2.0) % 80.0,
                memory_percent: (capability.id.len() as f64 * 3.0) % 70.0,
                network_mbps: (capability.id.len() as f64 * 5.0) % 100.0,
                storage_iops: (capability.id.len() as u64 * 100) % 1000,
            },
            quality_score: 0.8 + (capability.id.len() as f64 * 0.01) % 0.2,
        };

        let snapshot = AvailabilitySnapshot {
            timestamp: Utc::now(),
            available: new_performance.error_rate_percent < 10.0,
            response_time_ms: new_performance.response_time_ms,
            quality_score: new_performance.quality_score,

        monitor.last_health_check = Utc::now();
        monitor.current_performance = new_performance;
        monitor.availability_history.push(snapshot);

        if monitor.availability_history.len() > config.performance_history_size {
            monitor.availability_history.remove(0);
        }

        monitor.status = Self::calculate_capability_status(
            &monitor.current_performance,
            &monitor.alert_thresholds,
        );
        debug!("📊 Updated monitor for capability {}", capability.id);
        Ok(())
    }

    fn calculate_capability_status(
        performance: &PerformanceMetrics,
        thresholds: &AlertThresholds,
    ) -> CapabilityStatus {

        if performance.error_rate_percent > thresholds.max_error_rate_percent * 2.0
            || performance.response_time_ms > thresholds.max_response_time_ms * 2
            || performance.quality_score < thresholds.min_quality_score * 0.5
        {
            CapabilityStatus::Critical

        else if performance.error_rate_percent > thresholds.max_error_rate_percent
            || performance.response_time_ms > thresholds.max_response_time_ms
            || performance.quality_score < thresholds.min_quality_score
            CapabilityStatus::Degraded
        } else {
            CapabilityStatus::Healthy
