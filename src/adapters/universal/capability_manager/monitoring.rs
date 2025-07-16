//! Real-time capability monitoring and performance tracking
//!
//! This module provides comprehensive monitoring capabilities for tracking
//! capability health, performance metrics, availability, and alert thresholds.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

use super::super::traits::*;
use super::config::CapabilityManagerConfig;
use crate::BearDogResult;

/// Real-time capability monitoring
#[derive(Debug, Clone)]
pub struct CapabilityMonitor {
    /// Unique identifier for the capability being monitored
    pub capability_id: String,
    /// Key identifying the provider offering this capability
    pub provider_key: String,
    /// Timestamp of the last health check
    pub last_health_check: DateTime<Utc>,
    /// Current performance metrics
    pub current_performance: PerformanceMetrics,
    /// Historical availability snapshots
    pub availability_history: Vec<AvailabilitySnapshot>,
    /// Alert thresholds for monitoring
    pub alert_thresholds: AlertThresholds,
    /// Current status of the capability
    pub status: CapabilityStatus,
}

/// Performance metrics for capability monitoring
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Average response time in milliseconds
    pub response_time_ms: u64,
    /// Throughput per second
    pub throughput_per_sec: u64,
    /// Error rate as percentage
    pub error_rate_percent: f64,
    /// Current resource utilization levels
    pub resource_utilization: ResourceUtilization,
    /// Overall quality score (0.0 to 1.0)
    pub quality_score: f64,
}

/// Resource utilization metrics
#[derive(Debug, Clone)]
pub struct ResourceUtilization {
    /// CPU utilization percentage
    pub cpu_percent: f64,
    /// Memory utilization percentage
    pub memory_percent: f64,
    /// Network utilization in megabits per second
    pub network_mbps: f64,
    /// Storage I/O operations per second
    pub storage_iops: u64,
}

/// Snapshot of capability availability at a point in time
#[derive(Debug, Clone)]
pub struct AvailabilitySnapshot {
    /// Timestamp when snapshot was taken
    pub timestamp: DateTime<Utc>,
    /// Whether the capability was available
    pub available: bool,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Quality score at this timestamp
    pub quality_score: f64,
}

/// Alert thresholds for capability monitoring
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// Maximum acceptable response time in milliseconds
    pub max_response_time_ms: u64,
    /// Minimum availability percentage required
    pub min_availability_percent: f64,
    /// Maximum error rate percentage allowed
    pub max_error_rate_percent: f64,
    /// Minimum quality score required
    pub min_quality_score: f64,
}

/// Current status of a capability
#[derive(Debug, Clone)]
pub enum CapabilityStatus {
    /// Capability is functioning normally
    Healthy,
    /// Capability is functioning but with reduced performance
    Degraded,
    /// Capability is in critical condition
    Critical,
    /// Capability is offline or unavailable
    Offline,
    /// Capability status is unknown
    Unknown,
}

impl CapabilityMonitor {
    /// Update capability monitor with new performance data
    pub async fn update_capability_monitor(
        monitors: &Arc<RwLock<HashMap<String, CapabilityMonitor>>>,
        provider_key: &str,
        capability: &Capability,
        config: &CapabilityManagerConfig,
    ) -> BearDogResult<()> {
        let mut monitors_guard = monitors.write().await;
        let monitor_key = format!("{}:{}", provider_key, capability.id);

        // Get or create monitor
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
                },
                status: CapabilityStatus::Unknown,
            });

        // Update performance metrics (mock implementation)
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

        // Create availability snapshot
        let snapshot = AvailabilitySnapshot {
            timestamp: Utc::now(),
            available: new_performance.error_rate_percent < 10.0,
            response_time_ms: new_performance.response_time_ms,
            quality_score: new_performance.quality_score,
        };

        // Update monitor
        monitor.last_health_check = Utc::now();
        monitor.current_performance = new_performance;
        monitor.availability_history.push(snapshot);

        // Maintain history size
        if monitor.availability_history.len() > config.performance_history_size {
            monitor.availability_history.remove(0);
        }

        // Update status
        monitor.status = Self::calculate_capability_status(
            &monitor.current_performance,
            &monitor.alert_thresholds,
        );

        debug!("📊 Updated monitor for capability {}", capability.id);
        Ok(())
    }

    /// Calculate capability status based on performance and thresholds
    fn calculate_capability_status(
        performance: &PerformanceMetrics,
        thresholds: &AlertThresholds,
    ) -> CapabilityStatus {
        if performance.error_rate_percent > thresholds.max_error_rate_percent * 2.0 {
            CapabilityStatus::Critical
        } else if performance.response_time_ms > thresholds.max_response_time_ms * 2 {
            CapabilityStatus::Critical
        } else if performance.quality_score < thresholds.min_quality_score * 0.5 {
            CapabilityStatus::Critical
        } else if performance.error_rate_percent > thresholds.max_error_rate_percent {
            CapabilityStatus::Degraded
        } else if performance.response_time_ms > thresholds.max_response_time_ms {
            CapabilityStatus::Degraded
        } else if performance.quality_score < thresholds.min_quality_score {
            CapabilityStatus::Degraded
        } else {
            CapabilityStatus::Healthy
        }
    }
}
