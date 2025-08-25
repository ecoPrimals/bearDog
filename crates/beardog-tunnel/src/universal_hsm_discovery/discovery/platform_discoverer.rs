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


/// Platform HSM Discoverer
///
/// Discovers platform-specific HSMs including TPM modules

use super::super::*;
use beardog_errors::BearDogResult;
use tracing::{debug, info};
/// Platform-specific HSM discoverer
#[derive(Debug)]
pub struct PlatformDiscoverer;
impl PlatformDiscoverer {}


    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }
    pub async fn discover(&self, config: &DiscoveryConfig) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("💻 Discovering Platform HSMs");
        let mut hsms = Vec::new();
        
        // TPM 2.0 Discovery
        if let Ok(tpm_hsm) = self.discover_tpm().await {
            hsms.push(tpm_hsm);
        }
        // Platform-specific HSM discovery
        if config.enable_platform_discovery {
            if let Ok(platform_hsms) = self.discover_platform_hsms().await {
                hsms.extend(platform_hsms);
            }
        info!("Found {} Platform HSMs", hsms.len());
        Ok(hsms)
    /// Discover TPM 2.0 modules
    async fn discover_tpm(&self) -> BearDogResult<DiscoveredHsm> {
        debug!("🔍 Checking for TPM 2.0");
        // Check for TPM device files
        let tpm_paths = ["/dev/tpm0", "/dev/tpmrm0"];
        for path in &tpm_paths {
            if std::path::Path::new(path).exists() {
                return Ok(DiscoveredHsm {
                    id: "tpm2".to_string(),
                    name: "TPM 2.0".to_string(),
                    hsm_type: HsmType::Platform,
                    connection_info: ConnectionInfo::Platform {
                        device_path: path.to_string(),
                    },
                    capabilities: vec![
                        HsmCapability::KeyGeneration,
                        HsmCapability::Signing,
                        HsmCapability::RandomGeneration,
                    ],
                    status: HsmStatus::Available,
                });
        Err(BearDogError::not_found("TPM 2.0 device not found"))
    /// Discover platform-specific HSMs
    async fn discover_platform_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("🔍 Checking for platform-specific HSMs");
        // Intel TXT/TPM discovery
        if self.check_intel_txt().await {
            hsms.push(DiscoveredHsm {
                id: "intel_txt".to_string(),
                name: "Intel TXT".to_string(),
                hsm_type: HsmType::Platform,
                connection_info: ConnectionInfo::Platform {
                    device_path: "/dev/txt".to_string(),
                },
                capabilities: vec![HsmCapability::Attestation],
                status: HsmStatus::Available,
            });
    /// Check for Intel TXT support
    async fn check_intel_txt(&self) -> bool {
        // Check for Intel TXT capability
        std::path::Path::new("/sys/kernel/security/tpm0").exists()
}
