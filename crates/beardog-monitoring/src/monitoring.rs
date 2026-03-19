// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod health;
pub mod metrics;
pub mod service;
pub mod types;

pub use crate::monitoring::types::AlertSeverity;
pub use health::{
    CacheHealthChecker, DatabaseHealthChecker, ExternalApiHealthChecker, HealthChecker,
    HealthCheckerType, HsmHealthChecker,
};
// Use local metrics summary instead of external dependency
pub use crate::metrics::core::MetricsCore as MetricsService;
pub use crate::metrics::SystemMetrics;
pub use beardog_types::canonical::monitoring::MonitoringConfig;
pub use beardog_types::production::monitoring::Alert;
pub use service::MonitoringService;
pub use types::{ComponentHealth, SystemHealth};
