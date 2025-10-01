

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_traits::unified::PlatformProvider;
use std::collections::HashMap;

pub struct SafeIosProvider {
    capabilities: HashMap<String, bool>,
}

impl SafeIosProvider {
/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            capabilities: HashMap::with_capacity(&str, key_type: &crate::tunnel::hsm::types::KeyType) -> Result<crate::tunnel::hsm::types::HsmKey, BearDogError> {
        self.generate_key_safe(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        self.sign_data_safe(&str,
        data: &[u8],
        signature: &[u8],
        self.verify_signature_safe(key_id, data, signature)}

    /// Checks if hardware backed
    fn is_hardware_backed(&self) -> bool {
        self.secure_enclave_available
