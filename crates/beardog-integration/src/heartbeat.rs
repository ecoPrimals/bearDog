// SPDX-License-Identifier: AGPL-3.0-only

//! # Heartbeat Service
//!
//! Continuous health monitoring and status reporting to Songbird UPA.
//!
//! ## Modern Concurrency
//! - Non-blocking interval-based execution
//! - System metrics collection using `sysinfo`
//! - Graceful degradation on UPA unavailability
//! - Cancellation-safe via `tokio::select!`
//!
//! ## Design Philosophy
//! **Fail-open**: If UPA is unreachable, log warnings but continue operation.
//! BearDog services should not fail just because discovery is down.

use std::sync::Arc;
use std::time::Duration;

use sysinfo::System;
use tokio::time::interval;
use tracing::{debug, error, info, warn};

use crate::upa_client::{LoadMetrics, UpaClient};
use beardog_errors::BearDogError;

/// Heartbeat service configuration
#[derive(Debug, Clone)]
pub struct HeartbeatConfig {
    /// Heartbeat interval
    pub interval: Duration,
    /// Service ID from UPA registration
    pub service_id: String,
    /// Authentication token
    pub token: String,
}

/// Heartbeat service for continuous UPA status updates
///
/// ## Usage
/// ```no_run
/// use std::sync::Arc;
/// use std::time::Duration;
/// use beardog_integration::{UpaClient, UpaClientConfig, HeartbeatService, HeartbeatConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let upa_client = Arc::new(UpaClient::new(UpaClientConfig::default())?);
///
/// let config = HeartbeatConfig {
///     interval: Duration::from_secs(30),
///     service_id: "service-123".to_string(),
///     token: "token-xyz".to_string(),
/// };
///
/// let service = HeartbeatService::new(upa_client, config);
/// service.start().await?;
/// # Ok(())
/// # }
/// ```
pub struct HeartbeatService {
    upa_client: Arc<UpaClient>,
    config: HeartbeatConfig,
    system: System,
}

impl HeartbeatService {
    /// Create a new heartbeat service
    pub fn new(upa_client: Arc<UpaClient>, config: HeartbeatConfig) -> Self {
        info!(
            interval_secs = config.interval.as_secs(),
            "💓 Initializing heartbeat service"
        );

        Self {
            upa_client,
            config,
            system: System::new_all(),
        }
    }

    /// Start the heartbeat loop
    ///
    /// ## Blocking
    /// Runs indefinitely until cancelled or error occurs.
    ///
    /// ## Cancellation
    /// Use `tokio::select!` to cancel:
    /// ```no_run
    /// # use beardog_integration::HeartbeatService;
    /// # async fn example(service: HeartbeatService) {
    /// tokio::select! {
    ///     result = service.start() => {
    ///         println!("Heartbeat stopped: {:?}", result);
    ///     }
    ///     _ = tokio::signal::ctrl_c() => {
    ///         println!("Shutting down...");
    ///     }
    /// }
    /// # }
    /// ```
    pub async fn start(mut self) -> Result<(), BearDogError> {
        info!("▶️  Starting heartbeat service");

        let mut ticker = interval(self.config.interval);
        let mut consecutive_failures = 0;
        const MAX_CONSECUTIVE_FAILURES: u32 = 10;

        loop {
            ticker.tick().await;

            match self.send_heartbeat_with_metrics().await {
                Ok(()) => {
                    consecutive_failures = 0;
                    debug!("✅ Heartbeat successful");
                }
                Err(e) => {
                    consecutive_failures += 1;
                    warn!(
                        consecutive_failures = consecutive_failures,
                        error = %e,
                        "⚠️  Heartbeat failed"
                    );

                    // Only fail after many consecutive failures
                    // (UPA might be temporarily unavailable)
                    if consecutive_failures >= MAX_CONSECUTIVE_FAILURES {
                        error!(
                            max_failures = MAX_CONSECUTIVE_FAILURES,
                            "❌ Too many consecutive heartbeat failures, stopping service"
                        );
                        return Err(e);
                    }

                    // Exponential backoff on failure
                    let backoff = Duration::from_secs(2_u64.pow(consecutive_failures.min(5)));
                    debug!(backoff_secs = backoff.as_secs(), "Backing off before retry");
                    tokio::time::sleep(backoff).await;
                }
            }
        }
    }

    /// Send heartbeat with current system metrics
    async fn send_heartbeat_with_metrics(&mut self) -> Result<(), BearDogError> {
        let metrics = self.collect_metrics();

        debug!(
            cpu_percent = metrics.cpu_percent,
            memory_percent = metrics.memory_percent,
            active_connections = metrics.active_connections,
            "📊 Collected system metrics"
        );

        self.upa_client
            .send_heartbeat(&self.config.service_id, &self.config.token, metrics)
            .await
    }

    /// Collect system metrics
    ///
    /// ## Modern System Info
    /// Uses `sysinfo` crate for cross-platform metrics:
    /// - CPU usage (percentage)
    /// - Memory usage (percentage)
    /// - Active connections (placeholder - would integrate with API server)
    fn collect_metrics(&mut self) -> LoadMetrics {
        // Refresh system information
        self.system.refresh_cpu();
        self.system.refresh_memory();

        // Calculate CPU usage (global)
        let cpu_percent = self.system.global_cpu_info().cpu_usage();

        // Calculate memory usage percentage
        let memory_percent =
            (self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0) as f32;

        // Active connections would come from API server state
        // For now, return placeholder
        let active_connections = 0;

        LoadMetrics {
            cpu_percent,
            memory_percent,
            active_connections,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::upa_client::UpaClientConfig;

    #[test]
    fn test_heartbeat_config() {
        let config = HeartbeatConfig {
            interval: Duration::from_secs(30),
            service_id: "test-service".to_string(),
            token: "test-token".to_string(),
        };

        assert_eq!(config.interval, Duration::from_secs(30));
    }

    #[test]
    fn test_metrics_collection() {
        let upa_client = Arc::new(UpaClient::new(UpaClientConfig::default()).unwrap());

        let config = HeartbeatConfig {
            interval: Duration::from_secs(30),
            service_id: "test".to_string(),
            token: "token".to_string(),
        };

        let mut service = HeartbeatService::new(upa_client, config);
        let metrics = service.collect_metrics();

        // CPU should be between 0-100
        assert!(metrics.cpu_percent >= 0.0 && metrics.cpu_percent <= 100.0);

        // Memory should be between 0-100
        assert!(metrics.memory_percent >= 0.0 && metrics.memory_percent <= 100.0);
    }
}
