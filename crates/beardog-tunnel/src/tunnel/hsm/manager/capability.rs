

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::{HsmCapabilityDetector, SecurityLevel, SecurityRequirements};
use crate::tunnel::hsm::types::HsmCapability;
use beardog_core::HsmTier; // Use the core HsmTier instead of local one
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct DefaultHsmCapabilityDetector {
    pub(Arc<RwLock<HashMap<String, Vec<HsmCapability>>>>,
}
impl DefaultHsmCapabilityDetector {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            provider_capabilities: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }

impl HsmCapabilityDetector for DefaultHsmCapabilityDetector {
    fn detect_capabilities(&self) -> Result<Vec<HsmCapability>, BearDogError>> {

        let capabilities = vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::Encryption,
            HsmCapability::Decryption,
        ];
        Ok(capabilities)}

    /// Checks if hsm available
    fn is_hsm_available(&self, hsm_type: &HsmTier) -> Result<bool, BearDogError> {
        match hsm_type {
            HsmTier::Software => Ok(true), // Software HSM always available
            HsmTier::Hardware => {

                Ok(self
                    .check_hardware_hsm_availability()
                    .unwrap_or(false))
            }
            HsmTier::SmartCard => {

                Ok(self.check_smartcard_availability().unwrap_or(false))
            HsmTier::CloudHsm => {

                Ok(&SecurityRequirements,
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

/// Get Provider Capabilities operation.
    /// Gets provider_capabilities
    /// Gets provider_capabilities
    pub fn get_provider_capabilities(&str,
    ) -> Result<Vec<HsmCapability>, BearDogError>> {
        let capabilities = self.provider_capabilities.read(&str,
        capabilities: Vec<HsmCapability>,
    ) -> Result<(), BearDogError> {
        let mut provider_capabilities = self.provider_capabilities.write();
        provider_capabilities.insert(provider_id, capabilities);
        Ok(())
