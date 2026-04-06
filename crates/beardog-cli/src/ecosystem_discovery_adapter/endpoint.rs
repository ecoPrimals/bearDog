// SPDX-License-Identifier: AGPL-3.0-or-later

//! Endpoint URL parsing and descriptor mapping for discovered primals.

use super::EcosystemDiscoveryAdapter;
use beardog_config::global::BEARDOG_CONFIG;
use beardog_core::ecosystem::primal_types::DiscoveredPrimal;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use beardog_types::canonical::discovery::{
    AuthenticationMethod, PerformanceProfile, ServiceEndpoint, UniversalCapabilityType,
    UniversalServiceDescriptor,
};
use std::collections::HashMap;

impl EcosystemDiscoveryAdapter {
    /// Convert `DiscoveredPrimal` to `UniversalServiceDescriptor`
    pub(crate) fn primal_to_descriptor(primal: &DiscoveredPrimal) -> UniversalServiceDescriptor {
        // Parse endpoint URL into components
        let url_parts = Self::parse_endpoint_url(&primal.endpoint.url);

        UniversalServiceDescriptor {
            service_id: primal.primal_id.clone(),
            capabilities: primal
                .capabilities
                .iter()
                .filter_map(Self::map_capability_type)
                .collect(),
            endpoint: ServiceEndpoint {
                protocol: url_parts.0,
                host: url_parts.1,
                port: url_parts.2,
                path: url_parts.3,
                parameters: HashMap::new(),
            },
            auth_method: AuthenticationMethod::None, // Default, discovered later
            performance_profile: PerformanceProfile::default(),
            trust_score: 0.5, // Default trust score, adjusted through interaction
        }
    }

    /// Parse endpoint URL into (protocol, host, port, path)
    ///
    /// Default TCP port when omitted comes from [`BEARDOG_CONFIG.network.ports`]. IPC (`unix` /
    /// `ipc`) uses port `0` and stores the socket path in `path`.
    pub(crate) fn parse_endpoint_url(url: &str) -> (String, String, u16, Option<String>) {
        let default_port = BEARDOG_CONFIG.network.ports.api_port;

        if let Some(protocol_end) = url.find("://") {
            let protocol = url[..protocol_end].to_string();
            let rest = &url[protocol_end + 3..];

            if (protocol.eq_ignore_ascii_case("unix") || protocol.eq_ignore_ascii_case("ipc"))
                && let Some(path) = Self::unix_ipc_path_from_rest(rest)
            {
                return (protocol.to_lowercase(), String::new(), 0, Some(path));
            }

            if let Some(path_start) = rest.find('/') {
                let host_port = &rest[..path_start];
                let path = rest[path_start..].to_string();

                if let Some(port_start) = host_port.find(':') {
                    let host = host_port[..port_start].to_string();
                    let port = host_port[port_start + 1..].parse().unwrap_or(default_port);
                    return (protocol, host, port, Some(path));
                }

                return (protocol, host_port.to_string(), default_port, Some(path));
            }

            return (protocol, rest.to_string(), default_port, None);
        }

        // Absolute socket path (PRIMAL IPC / biomeos layout)
        if url.starts_with('/')
            && std::path::Path::new(url)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("sock"))
        {
            return ("unix".to_string(), String::new(), 0, Some(url.to_string()));
        }

        // Fallback: host or address without scheme (legacy HTTP-style discovery)
        ("http".to_string(), url.to_string(), default_port, None)
    }

    /// Strip `unix://` / `ipc://` authority and return the path segment for `unix:///path/to.sock`.
    fn unix_ipc_path_from_rest(rest: &str) -> Option<String> {
        let mut s = rest;
        if s.starts_with("//") {
            s = &s[2..];
            if let Some(pos) = s.find('/') {
                s = &s[pos..];
            } else {
                return None;
            }
        }
        if s.starts_with('/') {
            Some(s.to_string())
        } else if s.is_empty() {
            None
        } else {
            Some(format!("/{s}"))
        }
    }

    /// Map `ServiceCapabilityType` to `UniversalCapabilityType`
    ///
    /// Modern pattern matching without unwraps
    ///
    /// Maps service capability types to universal capability types.
    /// This is intentionally conservative to avoid false matches.
    const fn map_capability_type(_cap: &ServiceCapabilityType) -> Option<UniversalCapabilityType> {
        // Conservative approach: No mapping without explicit capability registry
        // This prevents false positives in capability matching
        // Future: Implement full capability registry for precise mapping
        None
    }
}
