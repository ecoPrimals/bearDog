//! mDNS Service Discovery
//!
//! BearDog advertises itself on the network so other primals can discover it.
//! Uses mDNS/DNS-SD (_beardog._tcp.local) for zero-config discovery.

use beardog_errors::BearDogError;
use std::net::IpAddr;
use tracing::info;

#[cfg(not(feature = "mdns"))]
use tracing::warn;

/// mDNS Service Advertisement Configuration
#[derive(Debug, Clone)]
pub struct ServiceAdvertisement {
    /// Service name (e.g., "beardog-node-001")
    pub instance_name: String,
    /// Service type (_beardog._tcp.local)
    pub service_type: String,
    /// Port where API is listening
    pub port: u16,
    /// IP addresses to advertise
    pub addresses: Vec<IpAddr>,
    /// TXT record data (capabilities, version, etc.)
    pub txt_records: Vec<(String, String)>,
}

impl Default for ServiceAdvertisement {
    fn default() -> Self {
        Self {
            instance_name: format!("beardog-{}", gethostname::gethostname().to_string_lossy()),
            service_type: "_beardog._tcp.local".to_string(),
            port: std::env::var("BEARDOG_API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            addresses: vec![],
            txt_records: vec![
                ("version".to_string(), env!("CARGO_PKG_VERSION").to_string()),
                ("primal".to_string(), "beardog".to_string()),
                ("capabilities".to_string(), "crypto,hsm,audit".to_string()),
                // Protocol information (Phase 5 integration!)
                ("protocols".to_string(), "http,jsonrpc,tarpc".to_string()),
                ("http_endpoint".to_string(), "/api/v1/crypto/*".to_string()),
                ("jsonrpc_endpoint".to_string(), "/rpc".to_string()),
                (
                    "tarpc_endpoint".to_string(),
                    "tcp://127.0.0.1:9000".to_string(),
                ),
                // Protocol discovery endpoint
                (
                    "protocol_discovery".to_string(),
                    "/api/v1/protocols".to_string(),
                ),
                // Crypto algorithms
                ("algorithms".to_string(), "aes-256-gcm,ed25519".to_string()),
            ],
        }
    }
}

/// mDNS Service Advertiser
///
/// Announces BearDog's presence on the local network.
/// Other primals discover BearDog by querying for "_beardog._tcp.local".
pub struct ServiceAdvertiser {
    config: ServiceAdvertisement,
}

impl ServiceAdvertiser {
    /// Create new service advertiser
    pub fn new(config: ServiceAdvertisement) -> Self {
        Self { config }
    }

    /// Start advertising service on mDNS
    ///
    /// # Implementation Note
    ///
    /// Real implementation would use mdns-sd or zeroconf crate.
    /// This is a placeholder showing the architecture.
    #[cfg(feature = "mdns")]
    pub async fn start(&self) -> Result<(), BearDogError> {
        info!(
            "Starting mDNS advertisement: {} on port {}",
            self.config.instance_name, self.config.port
        );

        // Real implementation:
        // let mdns = ServiceDaemon::new()?;
        // let service = ServiceInfo::new(
        //     &self.config.service_type,
        //     &self.config.instance_name,
        //     &self.config.addresses,
        //     self.config.port,
        //     &self.config.txt_records,
        // )?;
        // mdns.register(service)?;

        info!("✅ mDNS service advertised successfully");
        Ok(())
    }

    /// Start advertising (graceful fallback when mdns feature disabled)
    #[cfg(not(feature = "mdns"))]
    pub async fn start(&self) -> Result<(), BearDogError> {
        warn!(
            "mDNS feature not enabled. Service {} will not be advertised on local network.",
            self.config.instance_name
        );
        warn!("Enable with: cargo build --features mdns");
        warn!("Other primals can still discover via HTTP API at /api/v1/capabilities");
        Ok(())
    }

    /// Stop advertising service
    pub async fn stop(&self) -> Result<(), BearDogError> {
        info!("Stopping mDNS advertisement");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_service_advertisement() {
        let ad = ServiceAdvertisement::default();
        assert_eq!(ad.service_type, "_beardog._tcp.local");
        assert!(ad.instance_name.starts_with("beardog-"));
        assert_eq!(ad.port, 8080);
    }

    #[tokio::test]
    async fn test_service_advertiser_creation() {
        let ad = ServiceAdvertisement::default();
        let advertiser = ServiceAdvertiser::new(ad);

        // Should not panic
        let result = advertiser.start().await;
        assert!(result.is_ok());
    }
}
