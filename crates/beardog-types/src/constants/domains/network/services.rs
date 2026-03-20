// SPDX-License-Identifier: AGPL-3.0-only

//! Service type labels and discovery timeouts.

/// Note: Use default_health_port(), default_metrics_port()
/// functions for environment-aware configuration
pub use super::timeouts::DNS_RESOLUTION_TIMEOUT;

/// Service types
pub const SERVICE_TYPE_API: &str = "api";
/// Configuration constant: service type metrics
pub const SERVICE_TYPE_METRICS: &str = "metrics";
/// Configuration constant: service type health
pub const SERVICE_TYPE_HEALTH: &str = "health";
