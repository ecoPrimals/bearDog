// SPDX-License-Identifier: AGPL-3.0-or-later

//! Service announcement - how this primal announces itself

pub use crate::config::AnnouncementConfig;
use crate::error::Result;
use crate::types::PrimalInfo;
use tracing::{debug, info};

/// Service announcer
pub struct Announcer {
    config: AnnouncementConfig,
    primal_info: PrimalInfo,
    service_registry_url: Option<String>,
}

impl Announcer {
    /// Builds an announcer that will publish [`PrimalInfo`] using the given [`AnnouncementConfig`].
    ///
    /// No network I/O occurs until [`Announcer::start`] is called.
    pub fn new(config: AnnouncementConfig, primal_info: PrimalInfo) -> Self {
        Self {
            config,
            primal_info,
            service_registry_url: None,
        }
    }

    /// Set the service registry URL (e.g. from `SERVICE_REGISTRY_URL` at startup).
    #[must_use]
    pub fn with_service_registry_url(mut self, url: Option<String>) -> Self {
        self.service_registry_url = url;
        self
    }

    /// Start announcing this primal's capabilities
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::Error`] when announcement transport setup fails.
    pub async fn start(&self) -> Result<()> {
        if !self.config.enabled {
            debug!("Announcement disabled");
            return Ok(());
        }

        info!(
            "Announcing primal '{}' with capabilities: {:?}",
            self.primal_info.primal_id,
            self.primal_info
                .capabilities
                .iter()
                .map(|c| c.capability_type.as_str())
                .collect::<Vec<_>>()
        );

        for method in &self.config.methods {
            match method.as_str() {
                "mdns" => self.announce_via_mdns().await?,
                "environment" => self.announce_via_environment().await?,
                "service_registry" => self.announce_via_service_registry().await?,
                _ => debug!("Unknown announcement method: {}", method),
            }
        }

        Ok(())
    }

    /// Announce service via mDNS
    ///
    /// Broadcasts service availability on local network using mDNS.
    /// Agnostic to primal type - announces capabilities generically.
    ///
    /// # Implementation Status
    /// Currently logs announcement intent. Full implementation requires `mdns` crate.
    ///
    /// # Future Enhancement
    /// - Broadcast _beardog._tcp.local or capability-specific service
    /// - Include TXT records with capability metadata
    /// - Periodic re-announcement with TTL
    /// - Graceful shutdown announcement
    async fn announce_via_mdns(&self) -> Result<()> {
        info!(
            "mDNS announcement for {} with capabilities: {} (full impl pending)",
            self.primal_info.primal_id,
            self.primal_info
                .capabilities
                .iter()
                .map(|c| c.capability_type.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );

        // Graceful no-op: service still functions via other discovery methods
        debug!("mDNS announcement pending full implementation");
        Ok(())
    }

    async fn announce_via_environment(&self) -> Result<()> {
        // Environment announcement happens externally (set env vars)
        info!(
            "To announce via environment, set:\n\
             PRIMAL_{}_ENDPOINT=\"{}\"\n\
             PRIMAL_{}_CAPABILITIES=\"{}\"",
            self.primal_info.primal_id.to_uppercase(),
            self.primal_info.endpoint.primary_url,
            self.primal_info.primal_id.to_uppercase(),
            self.primal_info
                .capabilities
                .iter()
                .map(|c| c.capability_type.as_str())
                .collect::<Vec<_>>()
                .join(",")
        );
        Ok(())
    }

    /// Announce service via service registry
    ///
    /// Registers service with centralized registry (Consul, etcd, etc.).
    /// Caller supplies the registry URL via [`Announcer::with_service_registry_url`].
    ///
    /// # Configuration
    /// Example: `http://consul:8500` or `http://etcd:2379`
    ///
    /// # Implementation Status
    /// Currently logs registration intent. Full implementation requires HTTP client.
    ///
    /// # Future Enhancement
    /// - POST registration to Consul/etcd/Kubernetes
    /// - Include health check endpoint
    /// - Periodic heartbeat/TTL refresh
    /// - Deregistration on shutdown
    async fn announce_via_service_registry(&self) -> Result<()> {
        if let Some(registry_url) = self.service_registry_url.as_ref() {
            info!(
                "Service registry configured at {}, registration for {} pending full implementation",
                registry_url, self.primal_info.primal_id
            );

            debug!(
                "Would register: id={}, endpoint={}, capabilities={}",
                self.primal_info.primal_id,
                self.primal_info.endpoint.primary_url,
                self.primal_info
                    .capabilities
                    .iter()
                    .map(|c| c.capability_type.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        } else {
            debug!(
                "Service registry URL not set for {}",
                self.primal_info.primal_id
            );
        }

        // Graceful no-op: service still functions via other discovery methods
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Announcer;
    use crate::config::AnnouncementConfig;
    use crate::types::{Capability, PrimalInfo, ServiceEndpoint};
    use std::collections::HashMap;

    fn sample_config(enabled: bool, methods: Vec<&str>) -> AnnouncementConfig {
        AnnouncementConfig {
            enabled,
            methods: methods.into_iter().map(String::from).collect(),
            announcement_interval_secs: 5,
            ttl_secs: 30,
            mdns: Default::default(),
        }
    }

    fn sample_primal(id: &str) -> PrimalInfo {
        PrimalInfo {
            primal_id: id.to_string(),
            primal_type: "test".to_string(),
            version: "0.0.1".to_string(),
            display_name: "Announce Test".to_string(),
            capabilities: vec![Capability {
                capability_type: "test.cap".to_string(),
                version: "1".to_string(),
                features: vec![],
                parameters: HashMap::new(),
            }],
            endpoint: ServiceEndpoint {
                primary_url: "http://127.0.0.1:9".to_string(),
                fallback_urls: vec![],
                use_tls: false,
                path_prefix: None,
            },
        }
    }

    #[tokio::test]
    async fn announcer_builder_and_disabled_start() {
        let a = Announcer::new(
            sample_config(false, vec!["mdns"]),
            sample_primal("inline-ann-1"),
        );
        a.start()
            .await
            .expect("disabled announcer should return Ok without I/O");
    }

    #[tokio::test]
    async fn announcer_covers_all_branches_inline() {
        let config = sample_config(
            true,
            vec!["mdns", "environment", "service_registry", "unknown_method"],
        );
        Announcer::new(config, sample_primal("inline-ann-2"))
            .with_service_registry_url(Some("http://registry:8500".to_string()))
            .start()
            .await
            .expect("combined announcement paths should complete");
    }
}
