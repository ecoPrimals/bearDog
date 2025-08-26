

use super::traits::PlatformProvider;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_errors::{BearDogError, BearDogResult};
use tracing::{info, warn};

pub struct SafeAndroidProvider {

    strongbox_available: bool,
}
impl SafeAndroidProvider {

    pub fn new() -> BearDogResult<Self> {
        let strongbox_available = Self::check_strongbox_availability();
        Ok(Self {
            strongbox_available,
        })
    }

    fn check_strongbox_availability() -> bool {
        if !cfg!(target_os = "android") {
            info!("Not on Android platform, StrongBox not available");
            return false;
        }

        info!("Simulating StrongBox availability check on Android");
        true

    async fn generate_key_safe(&self, _key_id: &str, _key_type: &KeyType) -> BearDogResult<HsmKey> {

        Err(BearDogError::NotImplemented {
            message: "Android StrongBox key generation not yet implemented safely".to_string(),

    async fn sign_data_safe(&self, _key_id: &str, _data: &[u8]) -> BearDogResult<Vec<u8>> {

            message: "Android StrongBox signing not yet implemented safely".to_string(),

    async fn verify_signature_safe(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> BearDogResult<bool> {

            message: "Android StrongBox verification not yet implemented safely".to_string(),}

impl PlatformProvider for SafeAndroidProvider {
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
        self.strongbox_available
