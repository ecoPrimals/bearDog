

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::traits::PlatformProvider;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_errors::BearDogError;
use tracing::{info, warn};

pub struct SafeAndroidProvider {

    strongbox_available: bool,
}
impl SafeAndroidProvider {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        let strongbox_available = Self::check_strongbox_availability(&str, _key_type: &KeyType) -> Result<HsmKey, BearDogError> {

        Err(BearDogError::NotImplemented {
            message: "Android StrongBox key generation not yet implemented safely".to_string(&str, _data: &[u8]) -> Result<Vec<u8>, BearDogError>> {

            message: "Android StrongBox signing not yet implemented safely".to_string(&str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {

            message: "Android StrongBox verification not yet implemented safely".to_string(&str, key_type: &KeyType) -> Result<HsmKey, BearDogError> {
        self.generate_key_safe(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        self.sign_data_safe(&str,
        data: &[u8],
        signature: &[u8],
        self.verify_signature_safe(key_id, data, signature)}

    /// Checks if hardware backed
    fn is_hardware_backed(&self) -> bool {
        self.strongbox_available
