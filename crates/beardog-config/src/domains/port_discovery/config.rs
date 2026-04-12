// SPDX-License-Identifier: AGPL-3.0-or-later

//! Port discovery configuration types and defaults.

use beardog_errors::process_env;
use serde::{Deserialize, Serialize};

use super::env::parse_u16_env;

/// Port discovery strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryStrategy {
    /// Use explicit configuration only (no discovery)
    ExplicitOnly,
    /// Query other primals for port usage
    PrimalQuery,
    /// Find any available port from system
    SystemQuery,
    /// Full discovery (primal + system)
    Full,
}

/// **Fallback** lower bound for scanning when `BEARDOG_PORT_DISCOVERY_MIN` is unset.
pub const FALLBACK_PORT_SCAN_MIN: u16 = 8000;

/// **Fallback** upper bound when `BEARDOG_PORT_DISCOVERY_MAX` is unset.
pub const FALLBACK_PORT_SCAN_MAX: u16 = 9000;

/// **Fallback** excluded ports (comma-separated override: `BEARDOG_PORT_DISCOVERY_EXCLUDE`).
pub const FALLBACK_EXCLUDED_DEV_PORTS: &[u16] = &[8000, 8888];

/// **Fallback** primal-discovery timeout (ms) when `BEARDOG_PORT_DISCOVERY_TIMEOUT_MS` is unset.
pub const FALLBACK_PORT_DISCOVERY_TIMEOUT_MS: u64 = 2000;

fn default_excluded_ports() -> Vec<u16> {
    FALLBACK_EXCLUDED_DEV_PORTS.to_vec()
}

/// Port discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDiscoveryConfig {
    /// Discovery strategy to use
    pub strategy: DiscoveryStrategy,
    /// Minimum port to consider (**fallback**: `FALLBACK_PORT_SCAN_MIN`; env: `BEARDOG_PORT_DISCOVERY_MIN`)
    pub min_port: u16,
    /// Maximum port to consider (**fallback**: `FALLBACK_PORT_SCAN_MAX`; env: `BEARDOG_PORT_DISCOVERY_MAX`)
    pub max_port: u16,
    /// Ports to avoid (**fallback**: `FALLBACK_EXCLUDED_DEV_PORTS`; env: `BEARDOG_PORT_DISCOVERY_EXCLUDE`)
    pub excluded_ports: Vec<u16>,
    /// Primal discovery timeout (**fallback**: `FALLBACK_PORT_DISCOVERY_TIMEOUT_MS`)
    pub discovery_timeout_ms: u64,
}

impl Default for PortDiscoveryConfig {
    fn default() -> Self {
        Self {
            strategy: DiscoveryStrategy::Full,
            min_port: FALLBACK_PORT_SCAN_MIN,
            max_port: FALLBACK_PORT_SCAN_MAX,
            excluded_ports: default_excluded_ports(),
            discovery_timeout_ms: FALLBACK_PORT_DISCOVERY_TIMEOUT_MS,
        }
    }
}

impl PortDiscoveryConfig {
    /// Load port discovery settings from environment variables.
    #[must_use]
    pub fn from_env() -> Self {
        let mut base = Self::default();
        base.min_port = parse_u16_env("BEARDOG_PORT_DISCOVERY_MIN", FALLBACK_PORT_SCAN_MIN);
        base.max_port = parse_u16_env("BEARDOG_PORT_DISCOVERY_MAX", FALLBACK_PORT_SCAN_MAX);
        base.discovery_timeout_ms = process_env::var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(FALLBACK_PORT_DISCOVERY_TIMEOUT_MS);
        base.excluded_ports = if let Ok(s) = process_env::var("BEARDOG_PORT_DISCOVERY_EXCLUDE") {
            s.split(',').filter_map(|p| p.trim().parse().ok()).collect()
        } else {
            default_excluded_ports()
        };
        base
    }
}
