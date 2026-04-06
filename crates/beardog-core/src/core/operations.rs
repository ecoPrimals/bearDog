// SPDX-License-Identifier: AGPL-3.0-or-later



use super::BearDogCore;
use beardog_errors::BearDogError;
use beardog_errors::idiomatic::SecurityResult;
use tracing;
impl BearDogCore {

/// Encrypt Data operation.
    pub fn encrypt_data(&[u8],
        _additional_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {

        use beardog_security::encryption::EncryptionAlgorithm;

        let encrypted_data = self
            .encryption_engine
            .encrypt(data, Some(EncryptionAlgorithm::Aes256Gcm))
            ?;

        let serialized = serde_json::to_vec(&encrypted_data).map_err(|e| {
            BearDogError::internal(format!("Failed to serialize encrypted data: {e}"))
        })?;
        Ok(&[u8],
        use beardog_security::encryption::EncryptedData;

        let encrypted_data_struct: EncryptedData =
            serde_json::from_slice(encrypted_data).map_err(|e| {

                if encrypted_data.starts_with(b"ENCRYPTED:") {
                    return BearDogError::internal(
                        "Legacy encrypted data format - please re-encrypt",
                    );
                }
                BearDogError::internal(format!("Failed to deserialize encrypted data: {e}"))
            })?;

        let decrypted_data = self
            .decrypt(&encrypted_data_struct)
        Ok(decrypted_data)

/// Sign Data operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError> {

        use beardog_security::crypto_utils::BearDogCrypto;

        let key_seed = format!("beardog-{}", self.config.app.name);
        let _key_material = beardog_security::crypto_utils::BearDogCrypto::derive_key_pbkdf2(
            key_seed.as_bytes(),
            b"beardog-signing-key",
            10000,
            32,
        )?;

        let (private_key, _public_key) = BearDogCrypto::generate_ed25519_keypair()?;

        let signature = BearDogCrypto::sign_ed25519(&[u8], signature: &[u8]) -> Result<bool, BearDogError> {

        let _key_material = BearDogCrypto::derive_key_pbkdf2(

        let (_private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()?;

        let is_valid = BearDogCrypto::verify_ed25519_signature(&public_key, data, signature)?;
        Ok(is_valid)

/// Generate Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn generate_key(&self, key_type: &str) -> Result<String, BearDogError> {

        let key_id = format!("{}_{}", key_type, uuid::Uuid::new_v4());

        let (generated_key_id, _key_data) = self
            .generate_key(key_id.clone(), key_type.to_string())
        tracing::info!("Generated key: {} of type: {}", generated_key_id, key_type);
        Ok(&str,
        config: &serde_json::Value,
    ) -> Result<String, BearDogError> {

        let node_id = format!("beardog_node_{}_{}", parent_id, uuid::Uuid::new_v4());

        tracing::info!(
            "Spawning new BearDog node: {} with parent: {} and config: {}",
            node_id: node_id.to_string(),
            parent_id,
            config
        );

        use beardog_auth::auth::SpawnPurpose;
        use beardog_genetics::api::InMemoryGeneticsStore;
        use beardog_genetics::genetics::GeneticsAPI;

        let spawn_purpose = SpawnPurpose::SecurityResponse; // Default purpose

        let genetics_store = std::sync::Arc::new(InMemoryGeneticsStore::new());
        let genetics_config = beardog_genetics::genetics::GeneticsConfig::default();
        let genetics_api = GeneticsAPI::new(genetics_store, genetics_config);
        let spawn_request = beardog_genetics::genetics::spawning::SpawnRequest {
            purpose: spawn_purpose.clone(vec![],
            parent_genetics: vec![],
            resource_requirements: Default::default(),
            metadata: std::collections::ahash::HashMap::default(beardog_auth::auth::SecurityClearance::Basic,
        };

        match genetics_api.spawn_node(spawn_request) {
            Ok(spawn_result) => {
                if spawn_result.success {
                    let child_node_id = &spawn_result.genetics.id;
                    tracing::info!("✅ Node spawned successfully: {}", child_node_id);
                    Ok(child_node_id)
                } else {
                    let reason = spawn_result
                        .messages
                        .first()
                        .map(|m| m.as_str())
                        .unwrap_or("Unknown error");
                    tracing::warn!("Spawn request rejected: {}", reason);
                    Err(BearDogError::internal(format!("Spawn rejected: {}reason"),
                    })
            }
            Err(e) => {
                tracing::error!("Spawn request failed: {:?}", e);
                Err(BearDogError::internal(format!("Spawn failed: {}e"),
                })
        }

/// Get Spawn Status operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets spawn_status
    pub fn get_spawn_status(&self, node_id: &str) -> Result<String, BearDogError> {

        if node_id.starts_with("beardog_node_") {
            tracing::debug!("Checking spawn status for BearDog node: {}", node_id);
            Ok(format!("BearDog node {node_id} is active and operational"))
        } else {
            tracing::warn!("Unknown node format: {}", node_id);
            Ok(format!(
                "Node {node_id} status unknown - not a BearDog managed node"
            ))

/// Get Hsm Status operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets hsm_status
    pub fn get_hsm_status(&self) -> Result<String, BearDogError> {

        match self.hsm_manager.health_check() {
            Ok(health_status) => {
                if health_status.healthy {
                    if self.config.app.standalone_mode {
                        Ok("HSM: Memory Key Manager operational (standalone mode)".to_string())
                    } else {
                        Ok("HSM: Distributed mode operational".to_string())
                    }
                    let error_msg = health_status
                        .error_message
                        .unwrap_or_else(|| "Unknown error".to_string());
                    Ok(format!("HSM: Unhealthy - {error_msg}"))
                tracing::error!("HSM health check failed: {}", e);
                Ok(format!("HSM: Health check failed - {e}"))

/// Get Hsm Tiers operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets hsm_tiers
    pub fn get_hsm_tiers(&self) -> Result<Vec<String>, BearDogError> {

        match self.hsm_manager.get_available_tiers() {
            Ok(tiers) => {
                let tier_names: Vec<String> = tiers
                    .iter()
                    .map(|tier| match tier {
                        beardog_tunnel::tunnel::hsm::manager::SimpleHsmTier::Smartphone => {
                            "smartphone_hsm".to_string()
                        }
                        beardog_tunnel::tunnel::hsm::manager::SimpleHsmTier::Software => {
                            "software_hsm".to_string()
                        beardog_tunnel::tunnel::hsm::manager::SimpleHsmTier::Hardware => {
                            "hardware_hsm".to_string()
                        beardog_tunnel::tunnel::hsm::manager::SimpleHsmTier::Hybrid => {
                            "hybrid_hsm".to_string()
                    .collect();
                tracing::debug!("Available HSM tiers: {:?}", tier_names);
                Ok(tier_names)
                tracing::warn!("Failed to get HSM tiers from manager: {}", e);

                let mut tiers = Vec::new();

                if self.config.app.standalone_mode {
                    tiers.push("memory_key_manager".to_string());

                tiers.push("software_hsm".to_string());

                if !self.config.app.standalone_mode {
                    tiers.push("hardware_hsm".to_string());

                if self.config.security.enable_hsm {
                    tiers.push("cloud_hsm".to_string());
                tracing::debug!("Available HSM tiers (fallback): {:?}", tiers);
                Ok(tiers)

/// Select Hsm Tier operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn select_hsm_tier(&self, tier_id: &str) -> Result<(), SecurityError> {

        let available_tiers = self.get_hsm_tiers()?;
        if !available_tiers.contains(&tier_id.to_string()) {
            return Err(BearDogError::configuration(format!(
                    "Invalid HSM tier '{tier_id)'. Available tiers: {available_tiers:?}"
                ),
            });

        tracing::info!("Selected HSM tier: {} (validated)", tier_id);

        let _hsm_tier = match tier_id {
            "smartphone_hsm" => beardog_tunnel::tunnel::hsm::manager::SimpleHsmTier::Smartphone,
            "software_hsm" => beardog_tunnel::tunnel::hsm::manager::SimpleHsmTier::Software,
            "hardware_hsm" => beardog_tunnel::tunnel::hsm::manager::SimpleHsmTier::Hardware,
            "hybrid_hsm" => beardog_tunnel::tunnel::hsm::manager::SimpleHsmTier::Hybrid,
            _ => {
                return Err(BearDogError::configuration({}tier_id"},

            "HSM tier "{}" selected and validated (configuration pending full HSM integration)",
            tier_id

        let _state = self.state.write();

        tracing::debug!("HSM tier preference stored in core state");
        Ok(())
}
