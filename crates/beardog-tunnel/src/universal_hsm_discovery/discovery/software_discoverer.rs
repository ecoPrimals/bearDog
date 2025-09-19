

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::super::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::*;
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Debug)]
pub struct SoftwareDiscoverer;
impl SoftwareDiscoverer {}

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
        debug!("💾 Discovering Software HSMs");
        let mut hsms = Vec::new();

        hsms.push(DiscoveredHsm {
            name: "BearDog Software HSM".to_string(),
            capabilities: self.get_beardog_capabilities(),
            connection_info: HsmConnectionInfo {
                endpoint: "memory://beardog-software-hsm".to_string(),
            metadata: std::collections::HashMap::from([
                ("implementation".to_string(), "beardog-native".to_string()),
                ("security_level".to_string(), "software".to_string()),
            ]),
        });

        if self.detect_softhsm()? {
            hsms.push(DiscoveredHsm {
                name: "SoftHSM v2".to_string(),
                capabilities: self.get_softhsm_capabilities(),
                connection_info: HsmConnectionInfo {
                    endpoint: "pkcs11://softhsm".to_string(),
                    authentication: Some(None,
                },
                health_status: HsmHealthStatus::Available,
                metadata: std::collections::HashMap::from([
                    ("implementation".to_string(), "softhsm2".to_string()),
                    ("interface".to_string(), "pkcs11".to_string()),
                ]),
            });
        }

        if self.detect_openssl_engines()? {
                name: "OpenSSL Engine".to_string(),
                capabilities: self.get_openssl_capabilities(),
                    endpoint: "openssl://engine".to_string(),
                    ("implementation".to_string(), "openssl".to_string()),
                    ("interface".to_string(), "engine".to_string()),
        info!("Found {} Software HSMs", hsms.len());
        Ok(hsms)


    fn detect_softhsm(&self) -> Result<bool, BearDogError> {

        debug!("Checking for SoftHSM installation");

        let common_paths = [
            "/usr/lib/softhsm/libsofthsm2.so",
            "/usr/local/lib/softhsm/libsofthsm2.so",
            "/opt/softhsm/lib/softhsm/libsofthsm2.so",
        ];
        for path in &common_paths {
            if std::path::Path::new({}", path);
                return Ok(true);
            }
        Ok(false)


    fn detect_openssl_engines(&self) -> Result<bool, BearDogError> {
        debug!("Checking for OpenSSL engine support");

        Ok(true)

    /// Gets beardog_capabilities
    fn get_beardog_capabilities(&self) -> HsmCapabilities {
        HsmCapabilities {
            supported_algorithms: vec![
                "AES-256-GCM".to_string(),
            hardware_backed: false,
            fips_certified: false,
            cc_certified: false,
            supports_key_generation: true,
            supports_key_import: true,
            supports_attestation: false,

    /// Gets softhsm_capabilities
    fn get_softhsm_capabilities(&self) -> HsmCapabilities {
                "AES-256".to_string(),
                "AES-128".to_string(),
                "ECDSA-P384".to_string(),

    /// Gets openssl_capabilities
    fn get_openssl_capabilities(&self) -> HsmCapabilities {
}
