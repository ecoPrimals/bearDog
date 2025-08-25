// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// HSM Discovery Engine
///
/// This module handles the actual discovery process for HSMs across different
/// interface types and environments. It provides automatic detection and
/// enumeration of available HSMs.

pub mod cloud_discoverer;
pub mod mobile_discoverer;
pub mod network_discoverer;
pub mod pkcs11_discoverer;
pub mod platform_discoverer;
pub mod software_discoverer;
pub mod usb_discoverer;
use super::*;
use beardog_errors::{BearDogError, BearDogResult};
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
impl DiscoveryEngine {}


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
        if config.enable_pkcs11_discovery {
            let mut hsms = self.discover_pkcs11_hsms(config).await?;
            all_hsms.append(&mut hsms);
            let mut hsms = self.discover_cloud_hsms(config).await?;
            let mut hsms = self.discover_mobile_hsms(config).await?;
            let mut hsms = self.discover_platform_hsms(config).await?;
            let mut hsms = self.discover_network_hsms(config).await?;
            let mut hsms = self.discover_usb_hsms(config).await?;
            let mut hsms = self.discover_software_hsms(config).await?;
        }
        info!("✅ Discovery completed, found {} HSMs", all_hsms.len());
        Ok(all_hsms)
    async fn discover_pkcs11_hsms(
        self.pkcs11_discoverer.discover(config).await
    async fn discover_cloud_hsms(
        self.cloud_discoverer.discover(config).await
    async fn discover_mobile_hsms(
        self.mobile_discoverer.discover(config).await
    async fn discover_platform_hsms(
        self.platform_discoverer.discover(config).await
    async fn discover_network_hsms(
        self.network_discoverer.discover(config).await
    async fn discover_usb_hsms(
        self.usb_discoverer.discover(config).await
    async fn discover_software_hsms(
        self.software_discoverer.discover(config).await
