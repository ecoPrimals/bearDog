

use super::traits::PlatformProvider;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_errors::{BearDogError, BearDogResult};
use tracing::{debug, info, warn};

pub struct SafeIosProvider {

    secure_enclave_available: bool,
}
impl SafeIosProvider {

    pub fn new() -> BearDogResult<Self> {
        let secure_enclave_available = Self::check_secure_enclave_availability();
        Ok(Self {
            secure_enclave_available,
        })
    }

    fn check_secure_enclave_availability() -> bool {
        if !cfg!(target_os = "ios") {
            info!("Not on iOS platform, Secure Enclave not available");
            return false;
        }
        info!("Simulating Secure Enclave availability check on iOS");
        true

    async fn generate_key_safe(&self, _key_id: &str, _key_type: &KeyType) -> BearDogResult<HsmKey> {

        Err(BearDogError::NotImplemented {
            message: "iOS Secure Enclave key generation not yet implemented safely".to_string(),

    async fn sign_data_safe(&self, _key_id: &str, _data: &[u8]) -> BearDogResult<Vec<u8>> {

            message: "iOS Secure Enclave signing not yet implemented safely".to_string(),

    async fn verify_signature_safe(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> BearDogResult<bool> {

            message: "iOS Secure Enclave verification not yet implemented safely".to_string(),}

impl PlatformProvider for SafeIosProvider {
    async fn generate_key(&self, key_id: &str, key_type: &KeyType) -> BearDogResult<HsmKey> {
        self.generate_key_safe(key_id, key_type).await}

    async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        self.sign_data_safe(key_id, data).await
    async fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
        self.verify_signature_safe(key_id, data, signature).await}

    fn is_hardware_backed(&self) -> bool {
        self.secure_enclave_available
