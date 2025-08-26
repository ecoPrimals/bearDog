

use beardog_errors::BearDogResult;
use beardog_types::canonical::hsm::HsmKey;
use beardog_types::canonical::KeyType;
use serde::{Deserialize, Serialize};
use beardog_traits::canonical::HsmProvider;

#[allow(async_fn_in_trait)]
pub trait ZeroCostHsmProvider: HsmProvider {

    type Config: Clone + Send + Sync;

    type Storage: Send + Sync;

    const PROVIDER_NAME: &'static str;

    const MAX_KEY_SIZE: usize;

    async fn generate_key_zero_cost(
        &self,
        key_type: KeyType,
        key_id: &str,
    ) -> BearDogResult<HsmKey>;

    async fn sign_data_zero_cost(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn verify_signature_zero_cost(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool>;

    async fn encrypt_data_zero_cost(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>>;

    async fn decrypt_data_zero_cost(
        &self,
        key_id: &str,
        encrypted_data: &[u8],
    ) -> BearDogResult<Vec<u8>>;

    async fn delete_key_zero_cost(&self, key_id: &str) -> BearDogResult<()>;

    async fn list_keys_zero_cost(&self) -> BearDogResult<Vec<String>>;

    async fn get_hsm_status_zero_cost(&self) -> BearDogResult<ZeroCostHsmStatus>;

    async fn update_config_zero_cost(&mut self, config: Self::Config) -> BearDogResult<()>;

    fn get_storage(&self) -> &Self::Storage;

    fn get_storage_mut(&mut self) -> &mut Self::Storage;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostHsmStatus {

    pub available: bool,

    pub active_keys: u32,

    pub memory_usage: u64,

    pub ops_per_second: f64,
}
