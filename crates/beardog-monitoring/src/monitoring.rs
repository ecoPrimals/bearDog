// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

/// Pluggable health checkers (DB, cache, HSM, external APIs).
pub mod health;
/// Metrics collection, unified system, and export helpers used by monitoring.
pub mod metrics;
/// [`MonitoringService`] orchestration, snapshots, and Prometheus integration.
pub mod service;
/// Shared DTOs for health, alerts, Prometheus config, and resource usage.
pub mod types;

pub use crate::monitoring::types::AlertSeverity;
pub use health::{
    CacheHealthChecker, DatabaseHealthChecker, ExternalApiHealthChecker, HealthChecker,
    HealthCheckerType, HsmHealthChecker,
};
// Use local metrics summary instead of external dependency
pub use crate::metrics::SystemMetrics;
pub use crate::metrics::core::MetricsCore as MetricsService;
pub use beardog_types::canonical::monitoring::MonitoringConfig;
pub use beardog_types::production::monitoring::Alert;
pub use service::MonitoringService;
pub use types::{ComponentHealth, SystemHealth};
