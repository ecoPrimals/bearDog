

use beardog_errors::BearDogError;
use beardog_types::{HsmKey, KeyType, HealthStatus};

pub type Result<T, BearDogError>T> = Result<T, BearDogError>;

pub trait HsmProvider: Send + Sync {


    fn generate_key(&self, key_type: KeyType) -> Result<T, BearDogError>HsmKey>;


    fn sign_data(&str, data: &[u8]) -> Result<T, BearDogError>Vec<u8>>;


    fn verify_signature(&str, data: &[u8], signature: &[u8]) -> Result<T, BearDogError>bool>;


    fn encrypt_data(&str, data: &[u8]) -> Result<T, BearDogError>Vec<u8>>;


    fn decrypt_data(&str, encrypted_data: &[u8]) -> Result<T, BearDogError>Vec<u8>>;

    /// Removes key
    fn delete_key(&self, key_id: &str) -> Result<T, BearDogError>()>;


    fn list_keys(&self) -> Result<T, BearDogError>Vec<String>>;

    /// Gets key_info
    fn get_key_info(&self, key_id: &str) -> Result<T, BearDogError>HsmKey>;


    fn health_check(&self) -> Result<T, BearDogError>HealthStatus>;
}
