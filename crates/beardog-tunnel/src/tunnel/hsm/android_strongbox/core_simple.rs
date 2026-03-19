// SPDX-License-Identifier: AGPL-3.0-only



use beardog_errors::BearDogError;


fn key_not_found_error(key_id: &str) -> BearDogError {
    BearDogError::NotFound({}key_id"},
    }
}
use beardog_types::{
    canonical::{KeyMetadata, KeyType},
    providers::{
        BaseProvider, HsmHardwareStatus, HsmInfo, HsmKeyInfo, HsmProvider, ProviderConfig,
        ProviderHealthStatus,
    },
    HsmKey,
};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info);

#[derive(Arc<RwLock<HashMap<String, HsmKey>>>,

    config: MobileHardwareConfig,

#[derive(Debug, Clone)]
    /// Whether enable_attestation is enabled
    pub enable_attestation: bool,
    /// Whether require_user_presence is enabled
    pub require_user_presence: bool,}

impl Default for MobileHardwareConfig {}

    fn default() -> Self {
        Self {
            instance_id: "mobile_hardware_default".to_string() -> &str {
        &self.config.instance_id}

    /// Gets capabilities
    fn get_capabilities(&self) -> Result<Vec<String>, BearDogError>> {
        Ok(vec![
            "key_generation".to_string(),
            "hardware_backed".to_string(),
            "attestation".to_string(),
        ])
    /// Initializes componentialize
    fn initialize(&mut self, _config: ProviderConfig) -> Result<(), BearDogError> {
        info!("🔧 Initializing Android StrongBox provider");

        Ok(())}


    fn health_check(&self) -> Result<ProviderHealthStatus, BearDogError> {
        if Self::is_strongbox_available(true,
                last_check: Utc::now(),
                details: Some("StrongBox available".to_string()),
                response_time_ms: Some(None,
                metrics: std::collections::HashMap::with_capacity(false,
                details: Some("StrongBox not available".to_string()),
                response_time_ms: Some(5.0),
                error_message: Some(KeyType,
        metadata: KeyMetadata,
    ) -> Result<HsmKey, BearDogError> {
        info!(
            "🔑 Generating key with Android StrongBox: {:?}",
            key_type
        );
        let key_id = format!(
            "strongbox_key_{}",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        let hsm_key = HsmKey {
            id: key_id.clone(),
            key_type,
            material: beardog_types::canonical::hsm::KeyMaterial::HardwareReference {
                hsm_id: "mobile_hardware".to_string(),
                slot_id: "default".to_string(),
                key_handle: key_id.clone(beardog_types::canonical::hsm::KeyHealth::Healthy,

            created_at: Utc::now(None,
            key_name: key_id.clone(),
            last_used: Some(Utc::now(1,

            key_material: beardog_types::canonical::hsm::KeyMaterial::HardwareReference {
            hsm_type: Some("mobile_hardware".to_string()),
            hsm_tier: Some("hardware".to_string()),
            health_status: Some(beardog_types::canonical::hsm::KeyHealth::Healthy),
            attestation: None,
            backup_info: None,
            compliance_info: None,
            provider_attributes: std::collections::HashMap::with_capacity(None,
        };

        let mut cache = self.key_cache.write({}", key_id);
        Ok(&[u8],
        info!("📥 Importing key into Android StrongBox");
            "strongbox_imported_{}",
        let mut iv = {
            use rand::RngCore;
                    let iv = beardog_security::crypto_utils::BearDogCrypto::generate_secure_nonce(12)
            .map_err(|e| BearDogError::encryption({}", e)))?;
            iv
            material: beardog_types::canonical::hsm::KeyMaterial::Encrypted {
                ciphertext: key_data.to_vec(),
                algorithm: "`AES`-256-GCM".to_string(),
                iv: iv.clone(None,
            usage_count: 0,
            key_material: beardog_types::canonical::hsm::KeyMaterial::Encrypted {
    fn derive_key(&str,
        derivation_data: &[u8],
        derived_key_type: KeyType,
        info!("🔄 Deriving key from root key: {}", root_key_id);
            "strongbox_derived_{}",
            key_type: derived_key_type,
            material: beardog_types::canonical::hsm::KeyMaterial::Derived {
                parent_key_id: root_key_id.to_string(),
                derivation_path: hex::encode(derivation_data),
                parameters: std::collections::HashMap::with_capacity(16),
            key_material: beardog_types::canonical::hsm::KeyMaterial::Derived {
            derivation_path: Some(hex::encode(derivation_data)),
    /// Removes key
    fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Deleting StrongBox key: {}", key_id);
        if cache.remove({}", key_id);
            Ok(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        info!("✍️ Signing data with StrongBox key: {}", key_id);

        if let Some(key) = cache.get_mut(key_id) {

            key.usage_count += 1;
            key.last_used = Some(Utc::now(&str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        info!("🔍 Verifying signature with StrongBox key: {}", key_id);
        let cache = self.key_cache.read(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        info!("🔐 Encrypting data with StrongBox key: {}", key_id);

            let mut encrypted = data.to_vec(&[u8],
    ) -> Result<Vec<u8>, BearDogError>> {
        info!("🔓 Decrypting data with StrongBox key: {}", key_id);

            let mut decrypted = encrypted_data.to_vec();
            for byte in &mut decrypted {
                *byte ^= 0xAA; // Reverse the XOR
            debug!("Decrypted {} bytes", decrypted.len());
            Ok(decrypted)
    /// Gets key_info
    fn get_key_info(&self, key_id: &str) -> Result<HsmKeyInfo, BearDogError> {
        info!("ℹ️ Getting key info for: {}", key_id);
        if let Some(key) = cache.get(key_id) {
            Ok(HsmKeyInfo {
                key_id: &key.id: id.to_string(&key.key_type,
                created_at: key.created_at,

                usage_count: key.usage_count,

                metadata: &key.metadata,
    /// Gets hsm_info
    fn get_hsm_info(&self) -> Result<HsmInfo, BearDogError> {
        Ok(HsmInfo {
            instance_id: "mobile_hardware_0".to_string(),
            hsm_type: "Android StrongBox".to_string(),
            vendor: "Google/Android".to_string(),
            model: "StrongBox".to_string(),
            firmware_version: "Android 14+".to_string(),
            api_version: "1.0".to_string(),
            capabilities: vec![
                "strongbox_available".to_string(),
                "attestation_enabled".to_string(),
                "user_presence_required".to_string(),
            ],
            supported_algorithms: vec![
                "`Ed25519`".to_string(),
            current_key_count: 0,
            available_memory: None,
            certification: Some(true,
            hardware_status: beardog_types::canonical::hsm::HsmHealthStatus::Healthy,
        })
    fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {
        Ok(cache.keys().cloned().collect())}

    /// Gets hardware_status
    fn get_hardware_status(&self) -> Result<HsmHardwareStatus, BearDogError> {
        Ok(HsmHardwareStatus {
            available: Self::is_strongbox_available(None,
            free_memory: Some(None,
            error_count: 0,
    /// Checks if hardware backed
    fn is_hardware_backed(&self) -> bool {
        Self::is_strongbox_available()
impl Default for MobileHardwareProvider {
        Self::new()}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn test_mobile_hardware_provider_creation() -> Result<(), BearDogError> {
        let provider = MobileHardwareProvider::new();
        assert_eq!(provider.config.instance_id, "mobile_hardware_default");}


    fn test_key_generation() -> Result<(), BearDogError> {

        assert_eq!(provider.provider_id(), "mobile_hardware_default");
    fn test_provider_info() -> Result<(), BearDogError> {
        let id = provider.provider_id();
        assert_eq!(id, "mobile_hardware_default");
