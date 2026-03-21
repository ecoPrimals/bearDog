// SPDX-License-Identifier: AGPL-3.0-only

//! Synchronous capability discovery from process environment (no async, no config file).
//!
//! Implements the same rules as [`crate::discovery::CapabilityDiscovery::find_by_capability`]
//! for the `"environment"` method so crates like `beardog-ipc` and `beardog-core` can resolve
//! endpoints without duplicating logic.

use crate::types::{Capability, DiscoveredService, HealthStatus, QoSMetrics, ServiceEndpoint};
use std::collections::HashMap;
use std::env::VarError;
use tracing::debug;

/// Default TTL when no [`crate::config::DiscoveryConfig`] is loaded (seconds).
pub const DEFAULT_ENV_DISCOVERY_TTL_SECS: u64 = 600;

/// Discover services that advertise a given capability using injected environment lookups.
///
/// For production, use [`discovered_services_from_environment_from_env`], which reads the
/// process environment via [`beardog_errors::process_env`].
#[must_use]
pub fn discovered_services_from_environment_with<G, V>(
    capability: &str,
    cache_ttl_secs: u64,
    mut get_var: G,
    vars: V,
) -> Vec<DiscoveredService>
where
    G: FnMut(&str) -> Result<String, VarError>,
    V: IntoIterator<Item = (String, String)>,
{
    let mut services = Vec::new();

    let capability_key = format!(
        "CAPABILITY_{}_ENDPOINT",
        capability.to_uppercase().replace('-', "_")
    );

    if let Ok(endpoint) = get_var(&capability_key) {
        debug!(
            "Found capability endpoint: {} = {}",
            capability_key, endpoint
        );

        services.push(DiscoveredService {
            id: format!("env-{capability}"),
            service_type: "unknown".to_string(),
            display_name: format!("Environment-discovered {capability} service"),
            endpoint: ServiceEndpoint {
                primary_url: endpoint,
                fallback_urls: vec![],
                use_tls: true,
                path_prefix: None,
            },
            capabilities: vec![Capability {
                capability_type: capability.to_string(),
                version: "unknown".to_string(),
                features: vec![],
                parameters: HashMap::new(),
            }],
            qos: QoSMetrics::default(),
            health: HealthStatus::Unknown,
            discovered_at: std::time::SystemTime::now(),
            ttl_secs: cache_ttl_secs,
            discovery_method: "environment".to_string(),
            metadata: HashMap::new(),
        });
    }

    for (key, value) in vars {
        if key.starts_with("PRIMAL_") && key.ends_with("_ENDPOINT") {
            let parts: Vec<&str> = key.split('_').collect();
            if parts.len() >= 3 {
                let primal_name = parts[1..parts.len() - 1].join("_");

                let cap_key = format!("PRIMAL_{primal_name}_CAPABILITIES");
                if let Ok(caps) = get_var(&cap_key) {
                    if caps.split(',').any(|c| c.trim() == capability) {
                        debug!("Found primal {} providing {}", primal_name, capability);

                        services.push(DiscoveredService {
                            id: format!("primal-{}", primal_name.to_lowercase()),
                            service_type: primal_name.to_lowercase(),
                            display_name: format!("{primal_name} Primal"),
                            endpoint: ServiceEndpoint {
                                primary_url: value.clone(),
                                fallback_urls: vec![],
                                use_tls: value.starts_with("https"),
                                path_prefix: None,
                            },
                            capabilities: caps
                                .split(',')
                                .map(|c| Capability {
                                    capability_type: c.trim().to_string(),
                                    version: "unknown".to_string(),
                                    features: vec![],
                                    parameters: HashMap::new(),
                                })
                                .collect(),
                            qos: QoSMetrics::default(),
                            health: HealthStatus::Unknown,
                            discovered_at: std::time::SystemTime::now(),
                            ttl_secs: cache_ttl_secs,
                            discovery_method: "environment".to_string(),
                            metadata: HashMap::new(),
                        });
                    }
                }
            }
        }
    }

    services
}

/// Discover services that advertise a given capability via [`beardog_errors::process_env`].
#[must_use]
pub fn discovered_services_from_environment_from_env(
    capability: &str,
    cache_ttl_secs: u64,
) -> Vec<DiscoveredService> {
    discovered_services_from_environment_with(
        capability,
        cache_ttl_secs,
        |k| beardog_errors::process_env::var(k),
        beardog_errors::process_env::vars(),
    )
}

/// Discover services that advertise a given capability via environment variables only.
///
/// Resolution order matches [`crate::discovery::CapabilityDiscovery`] environment discovery:
/// 1. `CAPABILITY_{UPPERCASE}_ENDPOINT` where the capability string uses `-` or `_` normalized to `_` in the key.
/// 2. `PRIMAL_*_ENDPOINT` paired with `PRIMAL_*_CAPABILITIES` listing this capability.
///
/// This is a thin wrapper around [`discovered_services_from_environment_from_env`].
#[must_use]
pub fn discovered_services_from_environment(
    capability: &str,
    cache_ttl_secs: u64,
) -> Vec<DiscoveredService> {
    discovered_services_from_environment_from_env(capability, cache_ttl_secs)
}

/// Normalize a discovery [`ServiceEndpoint::primary_url`] to a Unix domain socket path for IPC.
///
/// Accepts `unix:///path`, `unix:/path`, or a bare filesystem path.
#[must_use]
pub fn primary_url_to_ipc_socket_path(primary_url: &str) -> String {
    let u = primary_url.trim();
    if let Some(rest) = u.strip_prefix("unix://") {
        return rest.to_string();
    }
    if let Some(rest) = u.strip_prefix("unix:") {
        return rest.to_string();
    }
    u.to_string()
}
