

use super::super::*;
use beardog_errors::BearDogError;
use tracing::debug;

use crate::tunnel::hsm::types::HsmCapabilities;
#[derive(Debug)]
pub struct MobileHsmCapabilityProber;
impl MobileHsmCapabilityProber {}

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
    pub async fn probe_android_strongbox_capabilities(
        &self,
        security_level: &str,
    ) -> Result<HsmCapabilities, BearDogError> {
        debug!("📱 Probing Android StrongBox: {}", security_level);

        Ok(HsmCapabilities::default())
    pub async fn probe_ios_secure_enclave_capabilities(
        enclave_version: &str,
        debug!("📱 Probing iOS Secure Enclave: {}", enclave_version);

}
