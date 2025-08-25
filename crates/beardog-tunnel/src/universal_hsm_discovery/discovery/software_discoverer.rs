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


/// Software HSM Discoverer
///
/// Discovers software-based HSMs and crypto providers

use super::super::*;
use beardog_errors::BearDogResult;
use beardog_types::canonical::hsm::*;
use std::collections::HashMap;
use tracing::{debug, info};
/// Software HSM discoverer
#[derive(Debug)]
pub struct SoftwareDiscoverer;
impl SoftwareDiscoverer {}


    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }
    pub async fn discover(&self, config: &DiscoveryConfig) -> BearDogResult<Vec<DiscoveredHsm>> {
        debug!("💾 Discovering Software HSMs");
        let mut hsms = Vec::new();
        
        // Always include BearDog's built-in software HSM
        hsms.push(DiscoveredHsm {
            name: "BearDog Software HSM".to_string(),
            hsm_type: HsmType::Software,
            provider: HsmProvider::BearDogSoftware,
            capabilities: self.get_beardog_capabilities(),
            connection_info: HsmConnectionInfo {
                endpoint: "memory://beardog-software-hsm".to_string(),
                authentication: None,
                tls_config: None,
            },
            health_status: HsmHealthStatus::Available,
            metadata: std::collections::HashMap::from([
                ("implementation".to_string(), "beardog-native".to_string()),
                ("security_level".to_string(), "software".to_string()),
            ]),
        });
        // Check for SoftHSM2 installation
        if self.detect_softhsm().await? {
            hsms.push(DiscoveredHsm {
                name: "SoftHSM v2".to_string(),
                hsm_type: HsmType::Software,
                provider: HsmProvider::SoftHSM,
                capabilities: self.get_softhsm_capabilities(),
                connection_info: HsmConnectionInfo {
                    endpoint: "pkcs11://softhsm".to_string(),
                    authentication: Some("PKCS#11 PIN".to_string()),
                    tls_config: None,
                },
                health_status: HsmHealthStatus::Available,
                metadata: std::collections::HashMap::from([
                    ("implementation".to_string(), "softhsm2".to_string()),
                    ("interface".to_string(), "pkcs11".to_string()),
                ]),
            });
        }
        // Check for OpenSSL engine support
        if self.detect_openssl_engines().await? {
                name: "OpenSSL Engine".to_string(),
                provider: HsmProvider::OpenSSLEngine,
                capabilities: self.get_openssl_capabilities(),
                    endpoint: "openssl://engine".to_string(),
                    authentication: None,
                    ("implementation".to_string(), "openssl".to_string()),
                    ("interface".to_string(), "engine".to_string()),
        info!("Found {} Software HSMs", hsms.len());
        Ok(hsms)
    
    /// Detect SoftHSM installation
    async fn detect_softhsm(&self) -> BearDogResult<bool> {
        // Check for SoftHSM library presence
        debug!("Checking for SoftHSM installation");
        // Common SoftHSM library locations
        let common_paths = [
            "/usr/lib/softhsm/libsofthsm2.so",
            "/usr/local/lib/softhsm/libsofthsm2.so",
            "/opt/softhsm/lib/softhsm/libsofthsm2.so",
        ];
        for path in &common_paths {
            if std::path::Path::new(path).exists() {
                debug!("Found SoftHSM at: {}", path);
                return Ok(true);
            }
        Ok(false)
    /// Detect OpenSSL engine support
    async fn detect_openssl_engines(&self) -> BearDogResult<bool> {
        debug!("Checking for OpenSSL engine support");
        // For now, assume OpenSSL is available if we can compile against it
        // In a real implementation, we'd check for specific engine availability
        Ok(true)
    /// Get BearDog software HSM capabilities}


    fn get_beardog_capabilities(&self) -> HsmCapabilities {
        HsmCapabilities {
            supported_algorithms: vec![
                "AES-256-GCM".to_string(),
                "AES-128-GCM".to_string(),
                "ChaCha20-Poly1305".to_string(),
                "ECDSA-P256".to_string(),
                "Ed25519".to_string(),
                "RSA-2048".to_string(),
                "RSA-4096".to_string(),
            ],
            max_key_size: 4096,
            hardware_backed: false,
            fips_certified: false,
            cc_certified: false,
            supports_key_generation: true,
            supports_key_import: true,
            supports_attestation: false,
    /// Get SoftHSM capabilities}


    fn get_softhsm_capabilities(&self) -> HsmCapabilities {
                "AES-256".to_string(),
                "AES-128".to_string(),
                "ECDSA-P384".to_string(),
    /// Get OpenSSL engine capabilities}


    fn get_openssl_capabilities(&self) -> HsmCapabilities {
}
