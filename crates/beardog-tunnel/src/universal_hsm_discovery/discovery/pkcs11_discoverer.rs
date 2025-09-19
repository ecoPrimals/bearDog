

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::super::*;
use beardog_errors::BearDogError;
use std::path::Path;
use tracing::{debug, info, warn};

use crate::tunnel::hsm::types::HsmCapabilities;

#[derive(Debug)]
pub struct Pkcs11Discoverer;
impl Pkcs11Discoverer {}

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
/// Discover operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("🔍 Discovering PKCS#11 HSMs");
        let mut hsms = Vec::new();

        let library_paths = self.get_common_pkcs11_paths();
        for path in library_paths {
            if Path::new({}", path);
                let hsm = DiscoveredHsm {
                    hsm_id: format!("pkcs11-{}", path), // path is already a String
                    name: format!("PKCS#11 HSM ({})", self.detect_vendor_from_path(crate::universal_hsm_discovery::HsmType::Hardware,
                    endpoint: crate::universal_hsm_discovery::HsmEndpoint {
                        address: path.clone(None,
                        protocol: "pkcs11".to_string(), // Will be determined by capability detection
                    health_status: HsmHealthStatus::healthy(),
                    discovered_at: chrono::Utc::now(),
                    last_health_check: chrono::Utc::now(IntegrationStatus::Discovered,
                };
                hsms.push(hsm);
            }
        }
        info!("Found {} PKCS#11 HSMs", hsms.len());
        Ok(hsms)
    /// Gets common_pkcs11_paths
    fn get_common_pkcs11_paths(&self) -> Vec<String> {
        vec![

            "/usr/lib/softhsm/libsofthsm2.so".to_string(),
            "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so".to_string(),
            "/usr/local/lib/softhsm/libsofthsm2.so".to_string(),
            "/opt/nfast/toolkits/pkcs11/libcknfast.so".to_string(),
            "/usr/lib/opencryptoki/libopencryptoki.so".to_string(),

            "C:\\Program Files\\SoftHSM2\\lib\\softhsm2-x64.dll".to_string(),
            "C:\\Windows\\System32\\eTPKCS11.dll".to_string(),
            "C:\\Windows\\System32\\dkck201.dll".to_string(),

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
