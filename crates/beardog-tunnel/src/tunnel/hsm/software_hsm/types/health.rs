// SPDX-License-Identifier: AGPL-3.0-or-later

//! Software HSM health snapshot and performance metrics wiring.

use beardog_errors::BearDogError;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::tunnel::hsm::types::{HsmHealthStatus, PerformanceMetrics};

/// Software health monitor
pub struct SoftwareHealthMonitor {
    /// Current health status
    pub health_status: Arc<RwLock<HsmHealthStatus>>,
    /// Performance metrics
    pub metrics: Arc<RwLock<PerformanceMetrics>>,
}

impl SoftwareHealthMonitor {
    /// Create a new software health monitor
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            health_status: Arc::new(RwLock::new(HsmHealthStatus {
                is_healthy: true,
                last_check: Utc::now(),
                error_message: None,
                performance_metrics: PerformanceMetrics {
                    operations_per_second: 0.0,
                    average_latency_ms: 0.0,
                    success_rate: 100.0,
                    memory_usage_mb: 0.0,
                    cpu_usage_percent: 0.0,
                    network_throughput_bps: 0.0,
                    latency_ms: 0.0,
                    throughput_mbps: 0.0,
                    uptime_seconds: 0,
                },
            })),
            metrics: Arc::new(RwLock::new(PerformanceMetrics {
                operations_per_second: 0.0,
                average_latency_ms: 0.0,
                success_rate: 100.0,
                memory_usage_mb: 0.0,
                cpu_usage_percent: 0.0,
                network_throughput_bps: 0.0,
                latency_ms: 0.0,
                throughput_mbps: 0.0,
                uptime_seconds: 0,
            })),
        })
    }

    /// Check health status
    ///
    /// # Errors
    /// Returns an error if health check fails
    pub async fn check_health(&self) -> Result<HsmHealthStatus, BearDogError> {
        let status = self.health_status.read().await;
        Ok(status.clone())
    }
}
