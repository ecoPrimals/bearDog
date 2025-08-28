

use super::super::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::*;
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Debug)]
pub struct SoftwareDiscoverer;
impl SoftwareDiscoverer {}

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
    pub async fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("💾 Discovering Software HSMs");
        let mut hsms = Vec::new();

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

    async fn detect_softhsm(&self) -> Result<bool, BearDogError> {

        debug!("Checking for SoftHSM installation");

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

    async fn detect_openssl_engines(&self) -> Result<bool, BearDogError> {
        debug!("Checking for OpenSSL engine support");

        Ok(true)

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

    fn get_softhsm_capabilities(&self) -> HsmCapabilities {
                "AES-256".to_string(),
                "AES-128".to_string(),
                "ECDSA-P384".to_string(),

    fn get_openssl_capabilities(&self) -> HsmCapabilities {
}
