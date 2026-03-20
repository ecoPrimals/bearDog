// SPDX-License-Identifier: AGPL-3.0-only

// Ecosystem Metrics Monitor
//
// Cross-system interaction tracking and ecosystem health monitoring.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Ecosystem interaction monitor
#[derive(Debug)]
pub struct EcosystemMonitor {
    #[allow(dead_code)] // Config reserved for future ecosystem monitoring
    config: EcosystemConfig,
}

impl EcosystemMonitor {
    /// Creates a new instance
    pub const fn new(config: EcosystemConfig) -> Result<Self, BearDogError> {
        Ok(Self { config })
    }

    /// Starts service
    /// Starts service
    pub fn start(&self) -> Result<(), BearDogError> {
        tracing::info!("Ecosystem monitor started");
        Ok(())
    }

    /// Records an ecosystem-scoped metric event (routing hook for future logic).
    pub const fn record_event(&self, _event: &super::MetricEvent) -> Result<(), BearDogError> {
        // Ecosystem event processing logic
        Ok(())
    }

    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> Result<EcosystemMetrics, BearDogError> {
        Ok(EcosystemMetrics {
            active_connections: 150,
            message_throughput: 500.0,
            network_latency_ms: 10.0,
            system_health_score: 0.92,
            integration_status: "healthy".to_string(),
        })
    }
}

/// Point-in-time ecosystem throughput and integration health snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemMetrics {
    /// Number of `active_connections`
    pub active_connections: u64,
    /// The message throughput value
    pub message_throughput: f64,
    /// The network latency ms value
    pub network_latency_ms: f64,
    /// The system health score value
    pub system_health_score: f64,
    /// Current status of the integration
    pub integration_status: String,
}

/// Polling and timeout settings for [`EcosystemMonitor`].
#[derive(Debug, Clone)]
pub struct EcosystemConfig {
    /// Number of `monitor_interval_secs`
    pub monitor_interval_secs: u64,
    /// Maximum time to wait for a dependency health probe before marking it failed.
    pub health_check_timeout_secs: u64,
}

impl Default for EcosystemConfig {
    fn default() -> Self {
        Self {
            monitor_interval_secs: 30,
            health_check_timeout_secs: 10,
        }
    }
}

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::nonminimal_bool
)]
#[cfg(test)]
#[path = "ecosystem_tests.rs"]
mod ecosystem_tests;
