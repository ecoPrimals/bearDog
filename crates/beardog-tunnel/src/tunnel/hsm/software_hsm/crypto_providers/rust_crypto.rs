

use super::super::types::CryptoProvider;
use crate::tunnel::hsm::types::KeyType;
use beardog_errors::BearDogError;

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

    pub async fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            config: RustCryptoConfig::default(),
        })

impl CryptoProvider for RustCryptoProvider {
    async fn generate_key(&self, key_type: &KeyType) -> Result<Vec<u8>, BearDogError>> {
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
    async fn sign(&self, key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError>> {

        Err(BearDogError::NotImplemented {
            message: "RustCrypto signing".to_string(),}

    async fn verify(&self, key: &[u8], data: &[u8], signature: &[u8]) -> Result<bool, BearDogError> {

            message: "RustCrypto verification".to_string(),
    async fn encrypt(&self, key_material: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>> {

            message: "RustCrypto encryption".to_string(),}

    async fn decrypt(&self, key_material: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>> {

            message: "RustCrypto decryption".to_string(),
    async fn initialize(&self) -> Result<(), BearDogError> {

        Ok(())}

    async fn generate_key_material(&self, key_type: &KeyType) -> Result<Vec<u8>, BearDogError>> {

                Ok(vec![0u8; 32]) // Placeholder
            KeyType::Secp256k1 => {

            _ => Err(BearDogError::unsupported_operation(format_args!("Key generation for {:?}", key_type).to_string(),
    async fn derive_key(
        &self,
        master_key: &[u8],
        derivation_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError>> {

        let mut derived_key = master_key.to_vec();
        derived_key.extend_from_slice(derivation_data);
        Ok(derived_key)
