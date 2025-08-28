

use super::{HsmCapabilityDetector, SecurityLevel, SecurityRequirements};
use crate::tunnel::hsm::types::HsmCapability;
use beardog_core::HsmTier; // Use the core HsmTier instead of local one
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct DefaultHsmCapabilityDetector {
    pub(crate) provider_capabilities: Arc<RwLock<HashMap<String, Vec<HsmCapability>>>>,
}
impl DefaultHsmCapabilityDetector {

    pub async fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            provider_capabilities: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }

impl HsmCapabilityDetector for DefaultHsmCapabilityDetector {
    async fn detect_capabilities(&self) -> Result<Vec<HsmCapability>, BearDogError>> {

        let capabilities = vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::Encryption,
            HsmCapability::Decryption,
        ];
        Ok(capabilities)}

    async fn is_hsm_available(&self, hsm_type: &HsmTier) -> Result<bool, BearDogError> {
        match hsm_type {
            HsmTier::Software => Ok(true), // Software HSM always available
            HsmTier::Hardware => {

                Ok(self
                    .check_hardware_hsm_availability()
                    .await
                    .unwrap_or(false))
            }
            HsmTier::SmartCard => {

                Ok(self.check_smartcard_availability().await.unwrap_or(false))
            HsmTier::CloudHsm => {

                Ok(self.check_cloud_hsm_availability().await.unwrap_or(false))
        }
    async fn recommend_hsm_tier(
        &self,
        requirements: &SecurityRequirements,
    ) -> Result<HsmTier, BearDogError> {

        match requirements.security_level {
            SecurityLevel::Basic => {

                Ok(HsmTier::Software)
            SecurityLevel::Medium => {

                Ok(HsmTier::SmartCard)
            SecurityLevel::High => {

                Ok(HsmTier::Hardware)
            SecurityLevel::Critical => {

                Ok(HsmTier::CloudHsm)

    pub async fn get_provider_capabilities(
        provider_id: &str,
    ) -> Result<Vec<HsmCapability>, BearDogError>> {
        let capabilities = self.provider_capabilities.read().await;
        Ok(capabilities.get(provider_id).cloned().unwrap_or_default())

    pub async fn update_provider_capabilities(
        provider_id: &str,
        capabilities: Vec<HsmCapability>,
    ) -> Result<(), BearDogError> {
        let mut provider_capabilities = self.provider_capabilities.write().await;
        provider_capabilities.insert(provider_id, capabilities);
        Ok(())
