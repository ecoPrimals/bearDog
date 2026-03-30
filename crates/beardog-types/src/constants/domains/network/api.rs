// SPDX-License-Identifier: AGPL-3.0-only

//! Re-exports for API-facing timeout and version constants.

pub use super::defaults::DEFAULT_HTTP_VERSION;
/// API-related network constants
/// Note: Use `default_api_port()`, `default_health_port()`, `default_metrics_port()`
/// functions for environment-aware configuration
pub use super::timeouts::{REQUEST_TIMEOUT, RESPONSE_TIMEOUT};
