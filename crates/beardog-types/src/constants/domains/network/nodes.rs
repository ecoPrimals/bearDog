// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node discovery and cluster port helpers.

pub use super::defaults::{DEFAULT_CONNECTION_POOL_SIZE, DEFAULT_MAX_CONNECTIONS};
pub use super::timeouts::{HEARTBEAT_TIMEOUT, KEEP_ALIVE_TIMEOUT};
use beardog_config::env_keys;

// Private fallback constants
const FALLBACK_NODE_DISCOVERY_PORT: u16 = 8090;
const FALLBACK_CLUSTER_PORT: u16 = 8091;

/// Get default node discovery port from environment or fallback to 8090
///
/// Checks `BEARDOG_NODE_DISCOVERY_PORT` environment variable first.
#[must_use]
pub fn default_node_discovery_port() -> u16 {
    std::env::var(env_keys::ENV_NODE_DISCOVERY_PORT)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(FALLBACK_NODE_DISCOVERY_PORT)
}

/// Get default cluster port from environment or fallback to 8091
///
/// Checks `BEARDOG_CLUSTER_PORT` environment variable first.
#[must_use]
pub fn default_cluster_port() -> u16 {
    std::env::var(env_keys::ENV_CLUSTER_PORT)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(FALLBACK_CLUSTER_PORT)
}

// Legacy const exports for backward compatibility (deprecated)
#[deprecated(
    since = "3.1.0",
    note = "Use default_node_discovery_port() for environment-aware configuration"
)]
/// Deprecated: use [`default_node_discovery_port`].
pub const DEFAULT_NODE_DISCOVERY_PORT: u16 = FALLBACK_NODE_DISCOVERY_PORT;

#[deprecated(
    since = "3.1.0",
    note = "Use default_cluster_port() for environment-aware configuration"
)]
/// Deprecated: use [`default_cluster_port`].
pub const DEFAULT_CLUSTER_PORT: u16 = FALLBACK_CLUSTER_PORT;
