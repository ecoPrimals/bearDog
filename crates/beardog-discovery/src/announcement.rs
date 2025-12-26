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
    pub fn new(config: AnnouncementConfig, primal_info: PrimalInfo) -> Self {
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

    async fn announce_via_mdns(&self) -> Result<()> {
        // TODO: Implement mDNS announcement
        debug!("mDNS announcement not yet implemented");
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

    async fn announce_via_service_registry(&self) -> Result<()> {
        // TODO: Implement service registry announcement
        debug!("Service registry announcement not yet implemented");
        Ok(())
    }
}

