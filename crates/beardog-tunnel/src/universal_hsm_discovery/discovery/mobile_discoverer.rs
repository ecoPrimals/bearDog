

use super::super::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::*;
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Debug)]
pub struct MobileDiscoverer;
impl MobileDiscoverer {}

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
    pub async fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        debug!("📱 Discovering Mobile HSMs");
        let mut hsms = Vec::new();

        if self.detect_android_strongbox().await? {
            hsms.push(DiscoveredHsm {
                name: "Android StrongBox".to_string(),
                hsm_type: HsmType::Hardware,
                provider: HsmProvider::AndroidStrongBox,
                capabilities: self.get_android_capabilities(),
                connection_info: HsmConnectionInfo {
                    endpoint: "android:strongbox".to_string(),
                    authentication: None,
                    tls_config: None,
                },
                health_status: HsmHealthStatus::Available,
                metadata: std::collections::HashMap::from([
                    ("platform".to_string(), "android".to_string()),
                    ("security_level".to_string(), "hardware".to_string()),
                ]),
            });
        }

        if self.detect_ios_secure_enclave().await? {
                name: "iOS Secure Enclave".to_string(),
                provider: HsmProvider::IOSSecureEnclave,
                capabilities: self.get_ios_capabilities(),
                    endpoint: "ios:secure_enclave".to_string(),
                    ("platform".to_string(), "ios".to_string()),
        info!("Found {} Mobile HSMs", hsms.len());
        Ok(hsms)

    async fn detect_android_strongbox(&self) -> Result<bool, BearDogError> {
        #[cfg(target_os = "android")]
        {

            debug!("Checking for Android StrongBox support");

            Ok(false)
        #[cfg(not(target_os = "android"))]

    async fn detect_ios_secure_enclave(&self) -> Result<bool, BearDogError> {}

        #[cfg(target_os = "ios")]

            debug!("Checking for iOS Secure Enclave support");

        #[cfg(not(target_os = "ios"))]

    fn get_android_capabilities(&self) -> HsmCapabilities {
        HsmCapabilities {
            supported_algorithms: vec!["AES-256-GCM".to_string(), "ECDSA-P256".to_string()],
            max_key_size: 256,
            hardware_backed: true,
            fips_certified: false,
            cc_certified: true,
            supports_key_generation: true,
            supports_key_import: false, // StrongBox typically doesn't allow key import
            supports_attestation: true,

    fn get_ios_capabilities(&self) -> HsmCapabilities {
            supported_algorithms: vec!["ECDSA-P256".to_string()],
            supports_key_import: false, // Secure Enclave doesn't allow key import
}
