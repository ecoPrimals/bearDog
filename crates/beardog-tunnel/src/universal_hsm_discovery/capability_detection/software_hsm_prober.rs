

use super::super::*;
use beardog_errors::BearDogResult;
use tracing::debug;

use crate::tunnel::hsm::types::HsmCapabilities;
#[derive(Debug)]
pub struct SoftwareHsmCapabilityProber;
impl SoftwareHsmCapabilityProber {}

    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }
    pub async fn probe_softhsm_capabilities(
        &self,
        config_path: &str,
    ) -> BearDogResult<HsmCapabilities> {
        debug!("🖥️ Probing SoftHSM: {}", config_path);

        Ok(HsmCapabilities::default())
    pub async fn probe_openssl_capabilities(
        engine_path: &str,
        debug!("🖥️ Probing OpenSSL engine: {}", engine_path);

    pub async fn probe_beardog_native_capabilities(
        instance_id: &str,
        debug!("🐻 Probing BearDog Native: {}", instance_id);

}
