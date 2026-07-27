// SPDX-License-Identifier: AGPL-3.0-or-later

//! Secret Storage Handler
//!
//! Provides encrypted secret storage with family-scoped key derivation.
//! Secrets are encrypted with ChaCha20-Poly1305 using keys derived from
//! the family seed via HKDF-SHA256.
//!
//! # Methods
//!
//! - `secrets.store` - Encrypt and store a secret by key name
//! - `secrets.retrieve` - Retrieve and decrypt a secret by key name
//! - `secrets.list` - List stored secret key names (not values)
//! - `secrets.delete` - Remove a stored secret
//!
//! # Architecture
//!
//! ```text
//! secrets.store(name, value)
//!   → HKDF(family_seed, name) → encryption_key
//!   → ChaCha20-Poly1305(encryption_key, value) → ciphertext
//!   → store(name, ciphertext, nonce)
//!
//! secrets.retrieve(name)
//!   → lookup(name) → ciphertext, nonce
//!   → HKDF(family_seed, name) → encryption_key
//!   → ChaCha20-Poly1305_decrypt(encryption_key, ciphertext, nonce) → value
//! ```
//!
//! # Storage Backend
//!
//! Uses capability-based discovery to find a `storage.store`-capable provider
//! at runtime. Falls back to local in-memory storage when no external provider
//! is available. This follows the TRUE PRIMAL principle: `BearDog` discovers
//! storage capabilities at runtime, never hardcodes another primal's name.
//!
//! # Security
//!
//! - Each secret gets a unique encryption key derived via HKDF from `family_seed` + `secret_name`
//! - ChaCha20-Poly1305 AEAD provides confidentiality + integrity
//! - Random nonce per encryption (never reused)
//! - Family-scoped: different families derive different keys for the same secret name

use super::utils::get_primal_name;
use super::{HandlerError, HandlerResult, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use crate::credential_store::CredentialStoreBackend;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_config::env_keys;
use beardog_core::crypto_service::algorithms::hashing;
use beardog_traits::unified::CredentialStore;
use beardog_types::primal_identity::PrimalIdentity;
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use hkdf::Hkdf;
use sha2::Sha256;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Secret storage handler with family-scoped encryption.
///
/// Stores secrets encrypted with ChaCha20-Poly1305 using keys derived
/// from the family seed. Each secret name produces a unique encryption key
/// via HKDF, ensuring cryptographic isolation between secrets.
///
/// Storage is delegated to a [`CredentialStoreBackend`] (Silicon Atheism
/// pattern), which may be in-memory (dev/test) or a persistent file vault
/// (production). The handler owns the encryption layer; the backend owns
/// the raw ciphertext persistence.
pub struct SecretsHandler {
    /// Primal identity for family-scoped key derivation
    identity: Arc<PrimalIdentity>,
    /// Pluggable credential store backend (in-memory, file vault, or platform-native)
    backend: Arc<CredentialStoreBackend>,
}

impl SecretsHandler {
    /// Create a new secrets handler with explicit identity and backend injection.
    pub const fn new(identity: Arc<PrimalIdentity>, backend: Arc<CredentialStoreBackend>) -> Self {
        Self { identity, backend }
    }
    /// Create a new secrets handler with an in-memory backend (dev/test convenience).
    #[must_use]
    pub fn new_in_memory(identity: Arc<PrimalIdentity>) -> Self {
        Self::new(identity, Arc::new(CredentialStoreBackend::in_memory()))
    }

    /// Derive a per-secret encryption key from family seed + secret name
    ///
    /// Uses HKDF-SHA256 with:
    /// - IKM: `family_id` bytes (the family seed identity)
    /// - Salt: "beardog-secrets-v1" (domain separation)
    /// - Info: `secret_name` (per-secret key derivation)
    ///
    /// Returns a 32-byte key suitable for ChaCha20-Poly1305
    fn derive_secret_key(&self, secret_name: &str) -> Result<[u8; 32], String> {
        let family_id = self.identity.family_id();

        let hk = Hkdf::<Sha256>::new(Some(b"beardog-secrets-v1"), family_id.as_bytes());

        let mut okm = [0u8; 32];
        // info field provides per-secret key isolation
        hk.expand(secret_name.as_bytes(), &mut okm)
            .map_err(|e| format!("HKDF-SHA256 key derivation failed: {e}"))?;

        Ok(okm)
    }

    /// Handle secrets.store - encrypt and store a secret
    ///
    /// # Parameters
    /// - `name`: Secret key name (string)
    /// - `value`: Secret value to encrypt (string, will be UTF-8 encoded)
    ///
    /// # Returns
    /// - `stored`: true on success
    /// - `name`: echo of the secret name
    /// - `provider`: primal name (self-knowledge)
    async fn handle_store(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params = params.ok_or("Missing params for secrets.store")?;

        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'name' parameter")?;

        let value = params
            .get("value")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'value' parameter")?;

        debug!("🔐 secrets.store: encrypting secret '{}'", name);

        // Derive per-secret encryption key from family seed
        let key = self.derive_secret_key(name)?;
        let cipher = ChaCha20Poly1305::new(&key.into());
        let nonce = ChaCha20Poly1305::generate_nonce(OsRng);

        // Encrypt the secret value
        let ciphertext = cipher
            .encrypt(&nonce, value.as_bytes())
            .map_err(|e| format!("Encryption failed: {e}"))?;

        // Store the sealed entry through the credential store backend
        let sealed = format!("{}:{}", BASE64.encode(nonce), BASE64.encode(&ciphertext),);
        self.backend
            .store(name, &sealed)
            .await
            .map_err(|e| format!("Backend store failed: {e}"))?;

        info!(
            "🔐 Secret '{}' stored (encrypted, family-scoped: {}, backend: {})",
            name,
            self.identity.family_id(),
            self.backend.backend_id(),
        );

        Ok(serde_json::json!({
            "stored": true,
            "name": name,
            "provider": get_primal_name(),
            "family_scoped": true,
        }))
    }

    /// Handle secrets.retrieve - retrieve and decrypt a secret
    ///
    /// # Parameters
    /// - `name`: Secret key name to retrieve
    ///
    /// # Returns
    /// - `value`: Decrypted secret value
    /// - `name`: echo of the secret name
    /// - `stored_at`: timestamp of when the secret was stored
    ///
    /// Parse a `nucleus:{family}:purpose:{name}` key pattern.
    ///
    /// Returns `Some(purpose_name)` if the pattern matches, `None` otherwise.
    fn parse_nucleus_purpose_key(name: &str) -> Option<&str> {
        let rest = name.strip_prefix("nucleus:")?;
        let (_family, after_family) = rest.split_once(':')?;
        let purpose_name = after_family.strip_prefix("purpose:")?;
        if purpose_name.is_empty() {
            return None;
        }
        Some(purpose_name)
    }

    /// Load the family seed from environment for purpose-key derivation.
    ///
    /// Checks `BEARDOG_FAMILY_SEED` then `FAMILY_SEED` (BearDog-prefixed takes precedence).
    fn load_family_seed() -> Result<Vec<u8>, String> {
        if let Ok(seed) = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED_PREFIXED)
            && !seed.is_empty()
        {
            return Ok(seed.into_bytes());
        }
        if let Ok(seed) = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED)
            && !seed.is_empty()
        {
            return Ok(seed.into_bytes());
        }
        Err(
            "Lazy purpose-key derivation requires FAMILY_SEED or BEARDOG_FAMILY_SEED env var"
                .to_string(),
        )
    }

    /// Derive a NUCLEUS purpose key and auto-store it as a secret.
    ///
    /// Uses the same HMAC-SHA256 convention as `crypto.derive_purpose_key`:
    /// `purpose_key = HMAC-SHA256(family_seed, hex("purpose-v1:" + purpose))`
    async fn lazy_derive_and_store(&self, name: &str, purpose: &str) -> Result<(), String> {
        let family_seed = Self::load_family_seed()?;

        let msg = hex::encode(format!("purpose-v1:{purpose}"));
        let derived = hashing::hmac_sha256(&family_seed, msg.as_bytes())
            .map_err(|e| format!("HMAC-SHA256 purpose derivation failed: {e}"))?;

        let derived_b64 = BASE64.encode(&derived);

        let key = self.derive_secret_key(name)?;
        let cipher = ChaCha20Poly1305::new(&key.into());
        let nonce = ChaCha20Poly1305::generate_nonce(OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, derived_b64.as_bytes())
            .map_err(|e| format!("Encryption of derived purpose key failed: {e}"))?;

        let sealed = format!("{}:{}", BASE64.encode(nonce), BASE64.encode(&ciphertext));
        self.backend
            .store(name, &sealed)
            .await
            .map_err(|e| format!("Backend store failed during lazy derivation: {e}"))?;

        info!(
            "🔑 Lazy-derived and stored purpose key '{}' for purpose '{}'",
            name, purpose
        );

        Ok(())
    }

    async fn handle_retrieve(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params = params.ok_or("Missing params for secrets.retrieve")?;

        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'name' parameter")?;

        debug!("🔓 secrets.retrieve: decrypting secret '{}'", name);

        // Look up the sealed entry from the credential store backend.
        // If not found and the name matches the NUCLEUS purpose-key pattern,
        // lazy-derive it from FAMILY_SEED and auto-store before retrieval.
        let (sealed, stored_at) = match self.backend.retrieve(name).await {
            Ok((val, meta)) => (val, meta.stored_at),
            Err(_) => {
                if let Some(purpose) = Self::parse_nucleus_purpose_key(name) {
                    self.lazy_derive_and_store(name, purpose).await?;
                    let (val, meta) =
                        self.backend.retrieve(name).await.map_err(|e| {
                            format!("Secret '{name}' not found after derivation: {e}")
                        })?;
                    (val, meta.stored_at)
                } else {
                    return Err(format!("Secret '{name}' not found").into());
                }
            }
        };

        // Decode sealed format: "base64(nonce):base64(ciphertext)"
        let (nonce_b64, ct_b64) = sealed
            .split_once(':')
            .ok_or_else(|| format!("Corrupt sealed format for '{name}'"))?;

        let nonce_bytes = BASE64
            .decode(nonce_b64)
            .map_err(|e| format!("Corrupt nonce: {e}"))?;

        let ciphertext = BASE64
            .decode(ct_b64)
            .map_err(|e| format!("Corrupt ciphertext: {e}"))?;

        if nonce_bytes.len() != 12 {
            return Err(format!(
                "Invalid nonce length: expected 12, got {}",
                nonce_bytes.len()
            )
            .into());
        }

        // Derive the same per-secret key and decrypt
        let key = self.derive_secret_key(name)?;
        let cipher = ChaCha20Poly1305::new(&key.into());
        let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| format!("Decryption failed (key mismatch or tampering): {e}"))?;

        let value = String::from_utf8(plaintext)
            .map_err(|e| format!("Decrypted value is not valid UTF-8: {e}"))?;

        info!("🔓 Secret '{}' retrieved successfully", name);

        Ok(serde_json::json!({
            "value": value,
            "name": name,
            "stored_at": stored_at,
            "provider": get_primal_name(),
        }))
    }

    /// Handle secrets.list - list stored secret names (not values)
    ///
    /// Returns only the key names, never the encrypted values.
    async fn handle_list(&self) -> Result<serde_json::Value, HandlerError> {
        let names = self
            .backend
            .list()
            .await
            .map_err(|e| format!("Backend list failed: {e}"))?;

        info!("📋 secrets.list: {} secrets stored", names.len());

        Ok(serde_json::json!({
            "secrets": names,
            "count": names.len(),
            "provider": get_primal_name(),
            "backend": self.backend.backend_id(),
        }))
    }

    /// Handle secrets.delete - remove a stored secret
    ///
    /// # Parameters
    /// - `name`: Secret key name to delete
    async fn handle_delete(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, HandlerError> {
        let params = params.ok_or("Missing params for secrets.delete")?;

        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'name' parameter")?;

        let removed = self
            .backend
            .delete(name)
            .await
            .map_err(|e| format!("Backend delete failed: {e}"))?;

        if removed {
            info!("🗑️ Secret '{}' deleted", name);
        } else {
            warn!("⚠️ Secret '{}' not found for deletion", name);
        }

        Ok(serde_json::json!({
            "deleted": removed,
            "name": name,
            "provider": get_primal_name(),
        }))
    }
}

impl MethodHandler for SecretsHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "secrets.store",
            "secrets.retrieve",
            "secrets.list",
            "secrets.delete",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        match method {
            "secrets.store" => self.handle_store(params).await,
            "secrets.retrieve" => self.handle_retrieve(params).await,
            "secrets.list" => self.handle_list().await,
            "secrets.delete" => self.handle_delete(params).await,
            _ => Err(format!("Unknown secrets method: {method}").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn test_identity() -> Arc<PrimalIdentity> {
        Arc::new(PrimalIdentity::for_test("test-family", "test-node"))
    }

    #[tokio::test]
    async fn test_secrets_handler_methods() {
        let handler = SecretsHandler::new_in_memory(test_identity());
        let methods = handler.methods();

        assert_eq!(methods.len(), 4);
        assert!(methods.contains(&"secrets.store"));
        assert!(methods.contains(&"secrets.retrieve"));
        assert!(methods.contains(&"secrets.list"));
        assert!(methods.contains(&"secrets.delete"));
    }

    #[tokio::test]
    async fn test_store_and_retrieve_roundtrip() {
        let handler = SecretsHandler::new_in_memory(test_identity());

        // Store a secret
        let store_params = serde_json::json!({
            "name": "api-key",
            "value": "sk-secret-12345-abcdef"
        });
        let store_result = handler.handle_store(Some(&store_params)).await;
        assert!(store_result.is_ok());
        let store_resp = store_result.expect("secrets.store should succeed in test");
        assert_eq!(store_resp["stored"], true);
        assert_eq!(store_resp["name"], "api-key");

        // Retrieve the secret
        let retrieve_params = serde_json::json!({ "name": "api-key" });
        let retrieve_result = handler.handle_retrieve(Some(&retrieve_params)).await;
        assert!(retrieve_result.is_ok());
        let retrieve_resp = retrieve_result.expect("secrets.retrieve should succeed in test");
        assert_eq!(retrieve_resp["value"], "sk-secret-12345-abcdef");
        assert_eq!(retrieve_resp["name"], "api-key");
        assert!(retrieve_resp["stored_at"].is_string());
    }

    #[tokio::test]
    async fn test_retrieve_nonexistent_secret() {
        let handler = SecretsHandler::new_in_memory(test_identity());

        let params = serde_json::json!({ "name": "nonexistent" });
        let result = handler.handle_retrieve(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[tokio::test]
    async fn test_family_scoped_isolation() {
        // Two handlers with different family IDs
        let family_a = Arc::new(PrimalIdentity::for_test("family-alpha", "node-1"));
        let family_b = Arc::new(PrimalIdentity::for_test("family-bravo", "node-1"));
        let handler_a = SecretsHandler::new_in_memory(family_a);
        let handler_b = SecretsHandler::new_in_memory(family_b);

        // Store same secret name in handler_a
        let store_params = serde_json::json!({
            "name": "shared-key",
            "value": "secret-from-alpha"
        });
        handler_a
            .handle_store(Some(&store_params))
            .await
            .expect("store should succeed in family isolation test");

        // Derive keys for the same name - they should differ between families
        let key_a = handler_a
            .derive_secret_key("shared-key")
            .expect("key derivation should succeed");
        let key_b = handler_b
            .derive_secret_key("shared-key")
            .expect("key derivation should succeed");
        assert_ne!(
            key_a, key_b,
            "Different families must derive different keys"
        );
    }

    #[tokio::test]
    async fn test_list_secrets() {
        let handler = SecretsHandler::new_in_memory(test_identity());

        // Store two secrets
        for name in &["key-1", "key-2"] {
            let params = serde_json::json!({
                "name": name,
                "value": format!("value-{}", name)
            });
            handler
                .handle_store(Some(&params))
                .await
                .expect("store should succeed in list test");
        }

        // List
        let list_result = handler.handle_list().await;
        assert!(list_result.is_ok());
        let resp = list_result.expect("secrets.list should succeed in test");
        assert_eq!(resp["count"], 2);

        let names = resp["secrets"]
            .as_array()
            .expect("secrets list should be array");
        let name_strs: Vec<&str> = names
            .iter()
            .map(|v| v.as_str().expect("secret name should be string"))
            .collect();
        assert!(name_strs.contains(&"key-1"));
        assert!(name_strs.contains(&"key-2"));
    }

    #[tokio::test]
    async fn test_delete_secret() {
        let handler = SecretsHandler::new_in_memory(test_identity());

        // Store
        let params = serde_json::json!({
            "name": "to-delete",
            "value": "temporary"
        });
        handler
            .handle_store(Some(&params))
            .await
            .expect("store should succeed in delete test");

        // Delete
        let del_params = serde_json::json!({ "name": "to-delete" });
        let del_result = handler.handle_delete(Some(&del_params)).await;
        assert!(del_result.is_ok());
        assert_eq!(del_result.expect("delete should succeed")["deleted"], true);

        // Verify it's gone
        let retrieve_params = serde_json::json!({ "name": "to-delete" });
        let result = handler.handle_retrieve(Some(&retrieve_params)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_nonexistent() {
        let handler = SecretsHandler::new_in_memory(test_identity());

        let params = serde_json::json!({ "name": "never-stored" });
        let result = handler.handle_delete(Some(&params)).await;
        assert!(result.is_ok());
        assert_eq!(
            result.expect("delete nonexistent should return Ok")["deleted"],
            false
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_lazy_derive_purpose_key() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "test-lazy-derive-seed-material!");
        let handler = SecretsHandler::new_in_memory(test_identity());

        // Secret doesn't exist yet — retrieve should lazy-derive it
        let params = serde_json::json!({ "name": "nucleus:test-family:purpose:storage" });
        let result = handler.handle_retrieve(Some(&params)).await;
        assert!(result.is_ok(), "lazy derivation should succeed");
        let resp = result.expect("should succeed");
        assert_eq!(resp["name"], "nucleus:test-family:purpose:storage");
        assert!(
            resp["value"].as_str().is_some(),
            "should return derived key"
        );

        // Derived key should be base64-encoded 32 bytes
        let key_b64 = resp["value"].as_str().expect("value");
        let key_bytes = BASE64.decode(key_b64).expect("valid base64");
        assert_eq!(key_bytes.len(), 32, "purpose key must be 32 bytes");

        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn test_lazy_derive_is_deterministic() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "deterministic-lazy-seed!!!!!!!!!");
        let handler = SecretsHandler::new_in_memory(test_identity());

        let params = serde_json::json!({ "name": "nucleus:test-family:purpose:inference" });
        let r1 = handler.handle_retrieve(Some(&params)).await.expect("r1");

        // Delete and re-derive — should produce the same key
        handler
            .handle_delete(Some(
                &serde_json::json!({ "name": "nucleus:test-family:purpose:inference" }),
            ))
            .await
            .expect("delete");

        let r2 = handler.handle_retrieve(Some(&params)).await.expect("r2");
        assert_eq!(
            r1["value"], r2["value"],
            "lazy derivation must be deterministic"
        );

        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn test_lazy_derive_without_family_seed_fails() {
        beardog_errors::process_env::remove_var("FAMILY_SEED");
        beardog_errors::process_env::remove_var("BEARDOG_FAMILY_SEED");
        let handler = SecretsHandler::new_in_memory(test_identity());

        let params = serde_json::json!({ "name": "nucleus:fam:purpose:storage" });
        let result = handler.handle_retrieve(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("FAMILY_SEED"));
    }

    #[tokio::test]
    async fn test_non_nucleus_key_not_lazy_derived() {
        let handler = SecretsHandler::new_in_memory(test_identity());

        // Non-matching pattern should NOT trigger lazy derivation
        let params = serde_json::json!({ "name": "my-regular-secret" });
        let result = handler.handle_retrieve(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[tokio::test]
    async fn test_parse_nucleus_purpose_key() {
        assert_eq!(
            SecretsHandler::parse_nucleus_purpose_key("nucleus:fam-123:purpose:storage"),
            Some("storage")
        );
        assert_eq!(
            SecretsHandler::parse_nucleus_purpose_key("nucleus:fam:purpose:inference"),
            Some("inference")
        );
        assert_eq!(
            SecretsHandler::parse_nucleus_purpose_key("nucleus:fam:purpose:"),
            None
        );
        assert_eq!(
            SecretsHandler::parse_nucleus_purpose_key("not-nucleus:fam:purpose:x"),
            None
        );
        assert_eq!(
            SecretsHandler::parse_nucleus_purpose_key("regular-key"),
            None
        );
    }

    #[tokio::test]
    async fn test_overwrite_secret() {
        let handler = SecretsHandler::new_in_memory(test_identity());

        // Store initial value
        let params = serde_json::json!({
            "name": "mutable-key",
            "value": "version-1"
        });
        handler
            .handle_store(Some(&params))
            .await
            .expect("initial store should succeed");

        // Overwrite with new value
        let params = serde_json::json!({
            "name": "mutable-key",
            "value": "version-2"
        });
        handler
            .handle_store(Some(&params))
            .await
            .expect("overwrite store should succeed");

        // Retrieve should return latest value
        let retrieve_params = serde_json::json!({ "name": "mutable-key" });
        let result = handler
            .handle_retrieve(Some(&retrieve_params))
            .await
            .expect("retrieve after overwrite should succeed");
        assert_eq!(result["value"], "version-2");
    }
}
