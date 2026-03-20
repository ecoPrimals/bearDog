// SPDX-License-Identifier: AGPL-3.0-only

// Network Domain Constants
//
// This module provides network-related constants consolidated from the large unified.rs file.
// It includes connection settings, timeouts, protocols, and network performance tuning.

pub mod addresses;
pub mod api;
pub mod circuit_breaker;
pub mod config;
pub mod defaults;
pub mod headers;
pub mod http;
pub mod intervals;
pub mod limits;
pub mod load_balancing;
pub mod nodes;
pub mod ports;
pub mod protocols;
pub mod rate_limiting;
pub mod services;
pub mod timeouts;

pub use addresses::DEFAULT_DNS_PORT;
// Note: Use default_api_port(), default_metrics_port(), default_metrics_bind() functions
// instead of the deprecated constants for environment-aware configuration
pub use limits::{MAX_CONNECTIONS, MAX_HEADER_SIZE};
pub use timeouts::{CONNECTION_TIMEOUT, REQUEST_TIMEOUT};

#[cfg(test)]
#[path = "../network_tests.rs"]
mod tests;
