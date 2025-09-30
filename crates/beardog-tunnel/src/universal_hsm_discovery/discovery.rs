

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod cloud_discoverer;
pub mod mobile_discoverer;
pub mod network_discoverer;
pub mod pkcs11_discoverer;
pub mod platform_discoverer;
pub mod software_discoverer;
pub mod usb_discoverer;
use super::*;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, error, info, warn};
pub use cloud_discoverer::CloudDiscoverer;
pub use mobile_discoverer::MobileDiscoverer;
pub use network_discoverer::NetworkDiscoverer;
pub use pkcs11_discoverer::Pkcs11Discoverer;
pub use platform_discoverer::PlatformDiscoverer;
pub use software_discoverer::SoftwareDiscoverer;
pub use usb_discoverer::UsbDiscoverer;

#[derive(Debug, Clone)]
    cloud_discoverer: CloudDiscoverer,
    mobile_discoverer: MobileDiscoverer,
    platform_discoverer: PlatformDiscoverer,
    network_discoverer: NetworkDiscoverer,
    usb_discoverer: UsbDiscoverer,
    software_discoverer: SoftwareDiscoverer,
}
impl DiscoveryEngine {}

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            pkcs11_discoverer: Pkcs11Discoverer::new()?,
            cloud_discoverer: CloudDiscoverer::new()?,
            mobile_discoverer: MobileDiscoverer::new()?,
            platform_discoverer: PlatformDiscoverer::new()?,
            network_discoverer: NetworkDiscoverer::new()?,
            usb_discoverer: UsbDiscoverer::new()?,
            software_discoverer: SoftwareDiscoverer::new(&DiscoveryConfig,
    ) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("🔍 Starting universal HSM discovery");
        let mut all_hsms = Vec::new();

        if config.enable_pkcs11_discovery {
            let mut hsms = self.discover_pkcs11_hsms(config)?;
            all_hsms.append(&mut hsms);
            let mut hsms = self.discover_cloud_hsms(config)?;
            let mut hsms = self.discover_mobile_hsms(config)?;
            let mut hsms = self.discover_platform_hsms(config)?;
            let mut hsms = self.discover_network_hsms(config)?;
            let mut hsms = self.discover_usb_hsms(config)?;
            let mut hsms = self.discover_software_hsms(config)?;
        }
        info!("✅ Discovery completed, found {} HSMs", all_hsms.len());
        Ok(all_hsms)
    fn discover_pkcs11_hsms(
        self.pkcs11_discoverer.discover(config)
    fn discover_cloud_hsms(
        self.cloud_discoverer.discover(config)
    fn discover_mobile_hsms(
        self.mobile_discoverer.discover(config)
    fn discover_platform_hsms(
        self.platform_discoverer.discover(config)
    fn discover_network_hsms(
        self.network_discoverer.discover(config)
    fn discover_usb_hsms(
        self.usb_discoverer.discover(config)
    fn discover_software_hsms(
        self.software_discoverer.discover(config)
