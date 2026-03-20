// SPDX-License-Identifier: AGPL-3.0-only

//! Service announcement - how this primal announces itself

pub use crate::config::AnnouncementConfig;
use crate::error::Result;
use crate::types::PrimalInfo;
use tracing::{debug, info};

/// Service announcer
pub struct Announcer {
    config: AnnouncementConfig,
    primal_info: PrimalInfo,
}

impl Announcer {
    /// Builds an announcer that will publish [`PrimalInfo`] using the given [`AnnouncementConfig`].
    ///
    /// No network I/O occurs until [`Announcer::start`] is called.
    pub const fn new(config: AnnouncementConfig, primal_info: PrimalInfo) -> Self {
        Self {
            config,
            primal_info,
        }
    }

    /// Start announcing this primal's capabilities
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
    /// Environment-driven configuration determines target registry.
    ///
    /// # Configuration
    /// Set `SERVICE_REGISTRY_URL` to enable registration.
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
        // Check if service registry is configured (environment-driven)
        if let Ok(registry_url) = std::env::var("SERVICE_REGISTRY_URL") {
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
                "Service registry not configured (set SERVICE_REGISTRY_URL) for {}",
                self.primal_info.primal_id
            );
        }

        // Graceful no-op: service still functions via other discovery methods
        Ok(())
    }
}
