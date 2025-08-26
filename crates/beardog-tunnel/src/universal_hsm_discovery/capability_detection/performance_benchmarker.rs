

use super::super::*;
use beardog_errors::BearDogResult;
use tracing::debug;

use crate::tunnel::hsm::types::HsmInterfaceType;
#[derive(Debug)]
pub struct PerformanceBenchmarker;
impl PerformanceBenchmarker {}

    pub fn new() -> BearDogResult<Self> {
        Ok(Self)
    }
    pub async fn benchmark_hsm_performance(
        &self,
        interface_type: &HsmInterfaceType,
    ) -> BearDogResult<PerformanceCapabilities> {
        debug!("⚡ Benchmarking HSM performance: {:?}", interface_type);

        Ok(PerformanceCapabilities::default())
}
