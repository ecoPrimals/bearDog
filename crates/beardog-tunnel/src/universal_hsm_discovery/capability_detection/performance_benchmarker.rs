

use super::super::*;
use beardog_errors::BearDogError;
use tracing::debug;

use crate::tunnel::hsm::types::HsmInterfaceType;
#[derive(Debug)]
pub struct PerformanceBenchmarker;
impl PerformanceBenchmarker {}

    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
    pub async fn benchmark_hsm_performance(
        &self,
        interface_type: &HsmInterfaceType,
    ) -> Result<PerformanceCapabilities, BearDogError> {
        debug!("⚡ Benchmarking HSM performance: {:?}", interface_type);

        Ok(PerformanceCapabilities::default())
}
