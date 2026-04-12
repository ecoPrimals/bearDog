// SPDX-License-Identifier: AGPL-3.0-or-later

//! Hierarchical port resolution: CLI → env → config → runtime discovery → default.

use beardog_errors::{BearDogError, process_env};

use super::config::{DiscoveryStrategy, PortDiscoveryConfig};
use super::discoverer::PortDiscoverer;

/// Discover port with full capability-based hierarchy
///
/// # Discovery Hierarchy
///
/// 1. CLI argument (if provided)
/// 2. Environment variable
/// 3. Config file value
/// 4. Runtime discovery (primals + system)
/// 5. Default constant
///
/// # Example
///
/// ```no_run
/// use beardog_config::domains::port_discovery::{discover_port_hierarchical, PortDiscoveryConfig};
///
/// # async fn example() {
/// // Discover API port with full hierarchy
/// let port = discover_port_hierarchical(
///     "BEARDOG_API_PORT",    // Environment variable
///     None,                  // No CLI override
///     None,                  // No config file value
///     8080,                  // Default fallback
///     PortDiscoveryConfig::default(),
/// ).await.expect("hierarchical port discovery");
/// # }
/// ```
///
/// # Errors
///
/// Returns [`BearDogError`] when automatic port probing fails or no valid port can be chosen.
pub async fn discover_port_hierarchical(
    env_var: &str,
    cli_override: Option<u16>,
    config_value: Option<u16>,
    default: u16,
    discovery_config: PortDiscoveryConfig,
) -> Result<u16, BearDogError> {
    // 1. CLI argument (highest priority - explicit human intent)
    if let Some(port) = cli_override {
        tracing::debug!("Using CLI override port: {}", port);
        return Ok(port);
    }

    // 2. Environment variable (human configuration)
    if let Ok(port_str) = process_env::var(env_var)
        && let Ok(port) = port_str.parse::<u16>()
    {
        tracing::debug!("Using environment variable {} = {}", env_var, port);
        return Ok(port);
    }

    // 3. Config file value (persistent configuration)
    if let Some(port) = config_value {
        tracing::debug!("Using config file port: {}", port);
        return Ok(port);
    }

    // 4. Runtime discovery (if enabled)
    if discovery_config.strategy != DiscoveryStrategy::ExplicitOnly {
        let discoverer = PortDiscoverer::new(discovery_config);
        match discoverer.discover().await {
            Ok(port) => {
                tracing::info!(
                    "Discovered available port via capability-based discovery: {}",
                    port
                );
                return Ok(port);
            }
            Err(e) => {
                tracing::warn!("Port discovery failed, falling back to default: {}", e);
            }
        }
    }

    // 5. Default constant (last resort)
    tracing::debug!("Using default port: {}", default);
    Ok(default)
}
