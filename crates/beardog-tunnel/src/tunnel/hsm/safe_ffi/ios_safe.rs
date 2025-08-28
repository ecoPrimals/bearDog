

use beardog_errors::BearDogError;
use beardog_traits::canonical::PlatformProvider;
use std::collections::HashMap;

pub struct SafeIosProvider {
    capabilities: HashMap<String, bool>,
}

impl SafeIosProvider {
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            capabilities: HashMap::new(),
        })
    }
}

impl PlatformProvider for SafeIosProvider {
    async fn generate_key(&self, key_id: &str, key_type: &crate::tunnel::hsm::types::KeyType) -> Result<crate::tunnel::hsm::types::HsmKey, BearDogError> {
        self.generate_key_safe(key_id, key_type).await}

    async fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        self.sign_data_safe(key_id, data).await
    async fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
        self.verify_signature_safe(key_id, data, signature).await}

    fn is_hardware_backed(&self) -> bool {
        self.secure_enclave_available
