// SPDX-License-Identifier: AGPL-3.0-only

//! Environment-driven configuration for [`super::EcosystemListener`].

/// Injected configuration for [`super::EcosystemListener`] (poll intervals and HTTP discovery targets).
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct EcosystemListenerEnvInputs {
    pub mdns_poll_interval_secs: u64,
    pub http_discovery_poll_interval_secs: u64,
    pub env_check_interval_secs: u64,
    pub mesh_discovery_interval_secs: u64,
    pub mdns_discovery_enabled: bool,
    pub discovery_base_port: u16,
    pub beardog_discovery_endpoint: Option<String>,
    pub ecosystem_discovery_endpoint: Option<String>,
    pub discovery_host: Option<String>,
    pub local_discovery_endpoint: Option<String>,
    pub http_discovery_timeout_secs: u64,
}

impl Default for EcosystemListenerEnvInputs {
    fn default() -> Self {
        use beardog_config::domains::network_ports::DEFAULT_API_PORT;
        Self {
            mdns_poll_interval_secs: 5,
            http_discovery_poll_interval_secs: 10,
            env_check_interval_secs: 15,
            mesh_discovery_interval_secs: 20,
            mdns_discovery_enabled: false,
            discovery_base_port: DEFAULT_API_PORT,
            beardog_discovery_endpoint: None,
            ecosystem_discovery_endpoint: None,
            discovery_host: None,
            local_discovery_endpoint: None,
            http_discovery_timeout_secs: 5,
        }
    }
}

impl EcosystemListenerEnvInputs {
    /// Read listener configuration from the process environment (read-only).
    #[must_use]
    pub fn from_env() -> Self {
        use beardog_types::canonical::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();
        let discovery_base_port = std::env::var("BEARDOG_DISCOVERY_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(network_config.service_ports.api_port);
        Self {
            mdns_poll_interval_secs: std::env::var("BEARDOG_MDNS_POLL_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            http_discovery_poll_interval_secs: std::env::var(
                "BEARDOG_HTTP_DISCOVERY_POLL_INTERVAL_SECS",
            )
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10),
            env_check_interval_secs: std::env::var("BEARDOG_ENV_CHECK_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(15),
            mesh_discovery_interval_secs: std::env::var("BEARDOG_MESH_DISCOVERY_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(20),
            mdns_discovery_enabled: std::env::var("BEARDOG_MDNS_DISCOVERY")
                .unwrap_or_else(|_| "false".to_string())
                == "true",
            discovery_base_port,
            beardog_discovery_endpoint: std::env::var("BEARDOG_DISCOVERY_ENDPOINT").ok(),
            ecosystem_discovery_endpoint: std::env::var("ECOSYSTEM_DISCOVERY_ENDPOINT").ok(),
            discovery_host: std::env::var("DISCOVERY_HOST").ok(),
            local_discovery_endpoint: std::env::var("LOCAL_DISCOVERY_ENDPOINT").ok(),
            http_discovery_timeout_secs: std::env::var("BEARDOG_ECOSYSTEM_LISTENER_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
        }
    }

    /// Resolved HTTP discovery URLs (primary + local fallback).
    #[must_use]
    pub fn discovery_endpoints(&self) -> Vec<String> {
        use beardog_config::global::BEARDOG_CONFIG;
        let bind_host = BEARDOG_CONFIG.network.api.bind_address.to_string();
        let primary = self.beardog_discovery_endpoint.clone().unwrap_or_else(|| {
            self.ecosystem_discovery_endpoint
                .clone()
                .unwrap_or_else(|| {
                    let discovery_host = self
                        .discovery_host
                        .clone()
                        .unwrap_or_else(|| "discovery.ecosystem.internal".to_string());
                    format!("http://{discovery_host}:{}", self.discovery_base_port)
                })
        });
        let local = self.local_discovery_endpoint.clone().unwrap_or_else(|| {
            format!("http://{bind_host}:{}/discovery", self.discovery_base_port)
        });
        vec![primary, local]
    }
}
