

use super::super::types::CryptoProvider;
use crate::tunnel::hsm::types::KeyType;
use beardog_errors::{BearDogError, BearDogResult};

pub struct RustCryptoProvider {

    config: RustCryptoConfig,
}
#[derive(Debug, Clone)]
pub struct RustCryptoConfig {
    pub algorithm_preferences: Vec<String>,
    pub security_level: String,}

impl Default for RustCryptoConfig {}

    fn default() -> Self {
        Self {
            algorithm_preferences: vec!["Ed25519".to_string(), "P256".to_string()],
            security_level: "high".to_string(),
        }
    }
impl RustCryptoProvider {

    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            config: RustCryptoConfig::default(),
        })

impl CryptoProvider for RustCryptoProvider {
    async fn generate_key(&self, key_type: &KeyType) -> BearDogResult<Vec<u8>> {
        match key_type {
            KeyType::Ed25519 => {

                Err(BearDogError::NotImplemented {
                    message: "Ed25519 key generation".to_string(),
                })
            }
            KeyType::P256 => {

                    message: "P256 key generation".to_string(),
            _ => Err(BearDogError::NotImplemented {
                message: format_args!("Key type {:?} not supported", key_type).to_string(),
            }),
    async fn sign(&self, key: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {

        Err(BearDogError::NotImplemented {
            message: "RustCrypto signing".to_string(),}

    async fn verify(&self, key: &[u8], data: &[u8], signature: &[u8]) -> BearDogResult<bool> {

            message: "RustCrypto verification".to_string(),
    async fn encrypt(&self, key_material: &[u8], plaintext: &[u8]) -> BearDogResult<Vec<u8>> {

            message: "RustCrypto encryption".to_string(),}

    async fn decrypt(&self, key_material: &[u8], ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {

            message: "RustCrypto decryption".to_string(),
    async fn initialize(&self) -> BearDogResult<()> {

        Ok(())}

    async fn generate_key_material(&self, key_type: &KeyType) -> BearDogResult<Vec<u8>> {

                Ok(vec![0u8; 32]) // Placeholder
            KeyType::Secp256k1 => {

            _ => Err(BearDogError::unsupported_operation(format_args!("Key generation for {:?}", key_type).to_string(),
    async fn derive_key(
        &self,
        master_key: &[u8],
        derivation_data: &[u8],
    ) -> BearDogResult<Vec<u8>> {

        let mut derived_key = master_key.to_vec();
        derived_key.extend_from_slice(derivation_data);
        Ok(derived_key)
