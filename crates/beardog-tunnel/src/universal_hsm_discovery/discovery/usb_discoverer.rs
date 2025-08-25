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


/// USB HSM Discoverer
///
/// Discovers USB-connected HSMs and security tokens

use super::super::*;
use beardog_errors::BearDogResult;
use tracing::{debug, info};
/// USB HSM discoverer
#[derive(Debug)]
pub struct UsbDiscoverer;
impl UsbDiscoverer {}


    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }
    pub async fn discover(&self, config: &DiscoveryConfig) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("🔌 Discovering USB HSMs");
        let mut hsms = Vec::new();
        
        if config.enable_usb_discovery {
            // YubiKey Discovery
            if let Ok(yubikeys) = self.discover_yubikeys().await {
                hsms.extend(yubikeys);
            }
            
            // SafeNet Token Discovery
            if let Ok(safenet_tokens) = self.discover_safenet_tokens().await {
                hsms.extend(safenet_tokens);
        }
        info!("Found {} USB HSMs", hsms.len());
        Ok(hsms)
    /// Discover YubiKey devices
    async fn discover_yubikeys(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("🔍 Checking for YubiKey devices");
        let mut yubikeys = Vec::new();
        // Check for YubiKey USB devices (simplified detection)
        // In a real implementation, this would use libusb or similar
        if self.check_yubikey_presence().await {
            yubikeys.push(DiscoveredHsm {
                id: "yubikey_usb".to_string(),
                name: "YubiKey".to_string(),
                hsm_type: HsmType::Usb,
                connection_info: ConnectionInfo::Usb {
                    vendor_id: 0x1050, // Yubico vendor ID
                    product_id: 0x0010, // Generic YubiKey product ID
                    serial_number: None,
                },
                capabilities: vec![
                    HsmCapability::KeyGeneration,
                    HsmCapability::Signing,
                    HsmCapability::Encryption,
                    HsmCapability::Authentication,
                ],
                status: HsmStatus::Available,
            });
        Ok(yubikeys)
    /// Discover SafeNet tokens
    async fn discover_safenet_tokens(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("🔍 Checking for SafeNet tokens");
        let mut tokens = Vec::new();
        // Check for SafeNet USB tokens
        if self.check_safenet_presence().await {
            tokens.push(DiscoveredHsm {
                id: "safenet_usb".to_string(),
                name: "SafeNet USB Token".to_string(),
                    vendor_id: 0x0529, // SafeNet vendor ID
                    product_id: 0x0600, // Generic SafeNet product ID
        Ok(tokens)
    /// Check for YubiKey presence (simplified)}


    async fn check_yubikey_presence(&self) -> bool {
        // In a real implementation, this would enumerate USB devices
        // For now, we'll check if the ykman command is available
        tokio::process::Command::new("which")
            .arg("ykman")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    /// Check for SafeNet token presence (simplified)
    async fn check_safenet_presence(&self) -> bool {
        // Check for SafeNet client tools or libraries
        std::path::Path::new("/usr/lib/libeToken.so").exists() ||
        std::path::Path::new("/opt/safenet").exists()
}
