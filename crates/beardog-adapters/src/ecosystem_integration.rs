

use super::universal::BearDogCapabilityAdapter;
use crate::EcosystemResult;
use beardog_errors::BearDogResult;

pub struct EcosystemIntegration {

    capability_adapter: BearDogCapabilityAdapter,
}
impl EcosystemIntegration {

    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            capability_adapter: BearDogCapabilityAdapter::new().await?,
        })
    }

    pub fn capability_adapter(&self) -> &BearDogCapabilityAdapter {
        &self.capability_adapter

    pub async fn register_capabilities(&self) -> BearDogResult<()> {

        tracing::info!("BearDog capabilities ready for service mesh registration");
        Ok(())

    pub async fn health_check(&self) -> EcosystemResult<bool> {
        Ok(true)
