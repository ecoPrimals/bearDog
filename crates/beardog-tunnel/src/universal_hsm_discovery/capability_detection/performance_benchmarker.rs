

use super::super::*;
use beardog_errors::BearDogError;
use tracing::debug;

use crate::tunnel::hsm::types::HsmInterfaceType;
#[derive(Debug, Clone)]
    ) -> Result<PerformanceCapabilities, BearDogError> {
        debug!("⚡ Benchmarking HSM performance: {:?}", interface_type);

        Ok(PerformanceCapabilities::default())
}
