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


/// PKCS#11 HSM Discoverer
///
/// Discovers PKCS#11 compatible HSMs by scanning for common library paths

use super::super::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::path::Path;
use tracing::{debug, info, warn};
// Import required types
use crate::tunnel::hsm::types::HsmCapabilities;
/// PKCS#11 HSM discoverer
#[derive(Debug)]
pub struct Pkcs11Discoverer;
impl Pkcs11Discoverer {}


    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }
    pub async fn discover(&self, config: &DiscoveryConfig) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("🔍 Discovering PKCS#11 HSMs");
        let mut hsms = Vec::new();
        // Common PKCS#11 library paths to check
        let library_paths = self.get_common_pkcs11_paths();
        for path in library_paths {
            if Path::new(&path).exists() {
                debug!("Found PKCS#11 library: {}", path);
                let hsm = DiscoveredHsm {
                    hsm_id: format!("pkcs11-{}", path), // path is already a String
                    name: format!("PKCS#11 HSM ({})", self.detect_vendor_from_path(&path)),
                    hsm_type: crate::universal_hsm_discovery::HsmType::Hardware,
                    endpoint: crate::universal_hsm_discovery::HsmEndpoint {
                        address: path.clone(), // path is already a String
                        port: None,
                        protocol: "pkcs11".to_string(),
                        secure: true,
                    },
                    capabilities: HsmCapabilities::default(), // Will be filled by capability detection
                    assigned_tier: HsmTier::BasicHardware, // Will be updated after capability detection
                    supports_human_entropy: false, // Will be determined by capability detection
                    health_status: HsmHealthStatus::healthy(),
                    discovered_at: chrono::Utc::now(),
                    last_health_check: chrono::Utc::now(),
                    integration_status: IntegrationStatus::Discovered,
                };
                hsms.push(hsm);
            }
        }
        info!("Found {} PKCS#11 HSMs", hsms.len());
        Ok(hsms)
    fn get_common_pkcs11_paths(&self) -> Vec<String> {
        vec![
            // Linux paths
            "/usr/lib/softhsm/libsofthsm2.so".to_string(),
            "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so".to_string(),
            "/usr/local/lib/softhsm/libsofthsm2.so".to_string(),
            "/opt/nfast/toolkits/pkcs11/libcknfast.so".to_string(),
            "/usr/lib/opencryptoki/libopencryptoki.so".to_string(),
            // Windows paths
            "C:\\Program Files\\SoftHSM2\\lib\\softhsm2-x64.dll".to_string(),
            "C:\\Windows\\System32\\eTPKCS11.dll".to_string(),
            "C:\\Windows\\System32\\dkck201.dll".to_string(),
            // macOS paths
            "/opt/homebrew/lib/softhsm/libsofthsm2.so".to_string(),
            "/Library/Application Support/Yubico/libykcs11.dylib".to_string(),
        ]}


    fn detect_vendor_from_path(&self, path: &str) -> String {
        let path_lower = path.to_lowercase();
        if path_lower.contains("softhsm") {
            "SoftHSM".to_string()
        } else if path_lower.contains("nfast") {
            "Entrust nShield".to_string()
        } else if path_lower.contains("opencryptoki") {
            "OpenCryptoki".to_string()
        } else if path_lower.contains("etoken") {
            "SafeNet eToken".to_string()
        } else if path_lower.contains("yubico") {
            "Yubico".to_string()
        } else {
            "Unknown".to_string()
}
