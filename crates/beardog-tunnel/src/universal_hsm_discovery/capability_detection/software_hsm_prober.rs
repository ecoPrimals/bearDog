

use super::super::*;
use beardog_errors::BearDogError;
use tracing::debug;

use crate::tunnel::hsm::types::HsmCapabilities;
#[derive(Debug, Clone)]
    ) -> Result<HsmCapabilities, BearDogError> {
        debug!("🖥️ Probing SoftHSM: {}", config_path);

        Ok(HsmCapabilities::default(&str,
        debug!("🖥️ Probing OpenSSL engine: {}", engine_path);

/// Probe Beardog Native Capabilities operation.
    pub fn probe_beardog_native_capabilities(&str,
        debug!("🐻 Probing BearDog Native: {}", instance_id);

}
