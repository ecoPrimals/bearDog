//! HSM Discovery Engine
//!
//! This module handles the actual discovery process for HSMs across different
//! interface types and environments. It provides automatic detection and
//! enumeration of available HSMs.

pub mod cloud_discoverer;
pub mod mobile_discoverer;
pub mod network_discoverer;
pub mod pkcs11_discoverer;
pub mod platform_discoverer;
pub mod software_discoverer;
pub mod usb_discoverer;

use super::*;
use beardog_errors::{BearDogResult, BearDogError};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tracing::{debug, error, info, warn};

pub use cloud_discoverer::CloudDiscoverer;
pub use mobile_discoverer::MobileDiscoverer;
pub use network_discoverer::NetworkDiscoverer;
pub use pkcs11_discoverer::Pkcs11Discoverer;
pub use platform_discoverer::PlatformDiscoverer;
pub use software_discoverer::SoftwareDiscoverer;
pub use usb_discoverer::UsbDiscoverer;

/// Main HSM discovery engine
#[derive(Debug)]
pub struct DiscoveryEngine {
    pkcs11_discoverer: Pkcs11Discoverer,
    cloud_discoverer: CloudDiscoverer,
    mobile_discoverer: MobileDiscoverer,
    platform_discoverer: PlatformDiscoverer,
    network_discoverer: NetworkDiscoverer,
    usb_discoverer: UsbDiscoverer,
    software_discoverer: SoftwareDiscoverer,
}

impl DiscoveryEngine {
    pub fn new() -> BearDogResult<Self> {
        Ok(Self {
            pkcs11_discoverer: Pkcs11Discoverer::new()?,
            cloud_discoverer: CloudDiscoverer::new()?,
            mobile_discoverer: MobileDiscoverer::new()?,
            platform_discoverer: PlatformDiscoverer::new()?,
            network_discoverer: NetworkDiscoverer::new()?,
            usb_discoverer: UsbDiscoverer::new()?,
            software_discoverer: SoftwareDiscoverer::new()?,
        })
    }

    /// Discover all available HSMs in the environment
    pub async fn discover_hsms(
        &self,
        config: &DiscoveryConfig,
    ) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔍 Starting universal HSM discovery");

        let mut all_hsms = Vec::new();

        // Execute discoveries sequentially for now (can be made concurrent later)
        if config.auto_discovery_enabled {
            let mut hsms = self.discover_pkcs11_hsms(config).await?;
            all_hsms.append(&mut hsms);

            let mut hsms = self.discover_cloud_hsms(config).await?;
            all_hsms.append(&mut hsms);

            let mut hsms = self.discover_mobile_hsms(config).await?;
            all_hsms.append(&mut hsms);

            let mut hsms = self.discover_platform_hsms(config).await?;
            all_hsms.append(&mut hsms);

            let mut hsms = self.discover_network_hsms(config).await?;
            all_hsms.append(&mut hsms);

            let mut hsms = self.discover_usb_hsms(config).await?;
            all_hsms.append(&mut hsms);

            let mut hsms = self.discover_software_hsms(config).await?;
            all_hsms.append(&mut hsms);
        }

        info!("✅ Discovery completed, found {} HSMs", all_hsms.len());
        Ok(all_hsms)
    }

    async fn discover_pkcs11_hsms(
        &self,
        config: &DiscoveryConfig,
    ) -> BearDogResult<Vec<DiscoveredHsm>> {
        self.pkcs11_discoverer.discover(config).await
    }

    async fn discover_cloud_hsms(
        &self,
        config: &DiscoveryConfig,
    ) -> BearDogResult<Vec<DiscoveredHsm>> {
        self.cloud_discoverer.discover(config).await
    }

    async fn discover_mobile_hsms(
        &self,
        config: &DiscoveryConfig,
    ) -> BearDogResult<Vec<DiscoveredHsm>> {
        self.mobile_discoverer.discover(config).await
    }

    async fn discover_platform_hsms(
        &self,
        config: &DiscoveryConfig,
    ) -> BearDogResult<Vec<DiscoveredHsm>> {
        self.platform_discoverer.discover(config).await
    }

    async fn discover_network_hsms(
        &self,
        config: &DiscoveryConfig,
    ) -> BearDogResult<Vec<DiscoveredHsm>> {
        self.network_discoverer.discover(config).await
    }

    async fn discover_usb_hsms(
        &self,
        config: &DiscoveryConfig,
    ) -> BearDogResult<Vec<DiscoveredHsm>> {
        self.usb_discoverer.discover(config).await
    }

    async fn discover_software_hsms(
        &self,
        config: &DiscoveryConfig,
    ) -> BearDogResult<Vec<DiscoveredHsm>> {
        self.software_discoverer.discover(config).await
    }
} 