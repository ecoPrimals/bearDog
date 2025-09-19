

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::super::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::*;
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Debug)]
pub struct MobileDiscoverer;
impl MobileDiscoverer {}

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
        debug!("📱 Discovering Mobile HSMs");
        let mut hsms = Vec::new();

        if self.detect_android_strongbox()? {
            hsms.push(DiscoveredHsm {
                name: "Android StrongBox".to_string(),
                capabilities: self.get_android_capabilities(),
                connection_info: HsmConnectionInfo {
                    endpoint: "android:strongbox".to_string(),
                metadata: std::collections::HashMap::from([
                    ("platform".to_string(), "android".to_string()),
                    ("security_level".to_string(), "hardware".to_string()),
                ]),
            });
        }

        if self.detect_ios_secure_enclave()? {
                name: "iOS Secure Enclave".to_string(),
                capabilities: self.get_ios_capabilities(),
                    endpoint: "ios:secure_enclave".to_string(),
                    ("platform".to_string(), "ios".to_string()),
        info!("Found {} Mobile HSMs", hsms.len());
        Ok(hsms)


    fn detect_android_strongbox(&self) -> Result<bool, BearDogError> {
        #[cfg(target_os = "android")]
        {

            debug!("Checking for Android StrongBox support");

            Ok(false)
        #[cfg(not(target_os = "android"))]


    fn detect_ios_secure_enclave(&self) -> Result<bool, BearDogError> {}

        #[cfg(target_os = "ios")]

            debug!("Checking for iOS Secure Enclave support");

        #[cfg(not(target_os = "ios"))]

    /// Gets android_capabilities
    fn get_android_capabilities(&self) -> HsmCapabilities {
        HsmCapabilities {
            supported_algorithms: vec!["AES-256-GCM".to_string(),
            hardware_backed: true,
            fips_certified: false,
            cc_certified: true,
            supports_key_generation: true,
            supports_key_import: false, // StrongBox typically doesn't allow key import
            supports_attestation: true,

    /// Gets ios_capabilities
    fn get_ios_capabilities(&self) -> HsmCapabilities {
            supported_algorithms: vec!["ECDSA-P256".to_string(), // Secure Enclave doesn't allow key import
}
