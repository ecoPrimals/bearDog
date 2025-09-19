

use super::super::*;
use beardog_errors::BearDogError;
use tracing::debug;

use crate::tunnel::hsm::types::HsmCapabilities;
#[derive(Debug, Clone)]
    ) -> Result<HsmCapabilities, BearDogError> {
        debug!("📱 Probing Android StrongBox: {}", security_level);

        Ok(HsmCapabilities::default(&str,
        debug!("📱 Probing iOS Secure Enclave: {}", enclave_version);

}
