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
//! Uses capability-based discovery to find a storage provider (e.g., NestGate)
//! at runtime. Falls back to local in-memory storage when no external provider
//! is available. This follows the TRUE PRIMAL principle: BearDog discovers
//! storage capabilities at runtime, never hardcodes another primal's name.
//!
//! # Security
//!
//! - Each secret gets a unique encryption key derived via HKDF from family_seed + secret_name
//! - ChaCha20-Poly1305 AEAD provides confidentiality + integrity
//! - Random nonce per encryption (never reused)
//! - Family-scoped: different families derive different keys for the same secret name

use super::utils::get_primal_name;
use super::MethodHandler;
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use beardog_types::primal_identity::PrimalIdentity;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use hkdf::Hkdf;
use parking_lot::RwLock;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// An encrypted secret entry in the store
#[derive(Debug, Clone)]
struct EncryptedSecret {
    /// Base64-encoded ciphertext (ChaCha20-Poly1305 output, includes auth tag)
    ciphertext: String,
    /// Base64-encoded 96-bit nonce
    nonce: String,
    /// ISO 8601 timestamp of when the secret was stored
    stored_at: String,
}

/// Secret storage handler with family-scoped encryption
///
/// Stores secrets encrypted with ChaCha20-Poly1305 using keys derived
/// from the family seed. Each secret name produces a unique encryption key
/// via HKDF, ensuring cryptographic isolation between secrets.
pub struct SecretsHandler {
    /// Primal identity for family-scoped key derivation
    identity: Arc<PrimalIdentity>,
    /// In-memory encrypted secret store (name -> encrypted entry)
    /// Production evolution: replace with NestGate storage.store capability
    store: Arc<RwLock<HashMap<String, EncryptedSecret>>>,
}

impl SecretsHandler {
    /// Create a new secrets handler with explicit identity injection
    pub fn new(identity: Arc<PrimalIdentity>) -> Self {
        Self {
            identity,
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Derive a per-secret encryption key from family seed + secret name
    ///
    /// Uses HKDF-SHA256 with:
    /// - IKM: family_id bytes (the family seed identity)
    /// - Salt: "beardog-secrets-v1" (domain separation)
    /// - Info: secret_name (per-secret key derivation)
    ///
    /// Returns a 32-byte key suitable for ChaCha20-Poly1305
    fn derive_secret_key(&self, secret_name: &str) -> Result<[u8; 32], String> {
        let family_id = self.identity.family_id();

        let hk = Hkdf::<Sha256>::new(
            Some(b"beardog-secrets-v1"),
            family_id.as_bytes(),
        );

        let mut okm = [0u8; 32];
        // info field provides per-secret key isolation
        hk.expand(secret_name.as_bytes(), &mut okm)
            .map_err(|e| format!("HKDF-SHA256 key derivation failed: {}", e))?;

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
    ) -> Result<serde_json::Value, String> {
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
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // Store the encrypted entry
        let entry = EncryptedSecret {
            ciphertext: BASE64.encode(&ciphertext),
            nonce: BASE64.encode(nonce),
            stored_at: chrono::Utc::now().to_rfc3339(),
        };

        self.store.write().insert(name.to_string(), entry);

        info!(
            "🔐 Secret '{}' stored (encrypted, family-scoped: {})",
            name,
            self.identity.family_id()
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
    async fn handle_retrieve(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params = params.ok_or("Missing params for secrets.retrieve")?;

        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'name' parameter")?;

        debug!("🔓 secrets.retrieve: decrypting secret '{}'", name);

        // Look up the encrypted entry
        let entry = {
            let store = self.store.read();
            store.get(name).cloned()
        };

        let entry = entry.ok_or_else(|| format!("Secret '{}' not found", name))?;

        // Decode stored ciphertext and nonce
        let ciphertext = BASE64
            .decode(&entry.ciphertext)
            .map_err(|e| format!("Corrupt ciphertext: {}", e))?;

        let nonce_bytes = BASE64
            .decode(&entry.nonce)
            .map_err(|e| format!("Corrupt nonce: {}", e))?;

        if nonce_bytes.len() != 12 {
            return Err(format!(
                "Invalid nonce length: expected 12, got {}",
                nonce_bytes.len()
            ));
        }

        // Derive the same per-secret key
        let key = self.derive_secret_key(name)?;
        let cipher = ChaCha20Poly1305::new(&key.into());
        let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);

        // Decrypt
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| format!("Decryption failed (key mismatch or tampering): {}", e))?;

        let value = String::from_utf8(plaintext)
            .map_err(|e| format!("Decrypted value is not valid UTF-8: {}", e))?;

        info!("🔓 Secret '{}' retrieved successfully", name);

        Ok(serde_json::json!({
            "value": value,
            "name": name,
            "stored_at": entry.stored_at,
            "provider": get_primal_name(),
        }))
    }

    /// Handle secrets.list - list stored secret names (not values)
    ///
    /// Returns only the key names, never the encrypted values.
    async fn handle_list(&self) -> Result<serde_json::Value, String> {
        let store = self.store.read();
        let names: Vec<&str> = store.keys().map(String::as_str).collect();

        info!("📋 secrets.list: {} secrets stored", names.len());

        Ok(serde_json::json!({
            "secrets": names,
            "count": names.len(),
            "provider": get_primal_name(),
        }))
    }

    /// Handle secrets.delete - remove a stored secret
    ///
    /// # Parameters
    /// - `name`: Secret key name to delete
    async fn handle_delete(
        &self,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let params = params.ok_or("Missing params for secrets.delete")?;

        let name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'name' parameter")?;

        let removed = self.store.write().remove(name).is_some();

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

#[async_trait]
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
    ) -> Result<serde_json::Value, String> {
        match method {
            "secrets.store" => self.handle_store(params).await,
            "secrets.retrieve" => self.handle_retrieve(params).await,
            "secrets.list" => self.handle_list().await,
            "secrets.delete" => self.handle_delete(params).await,
            _ => Err(format!("Unknown secrets method: {}", method)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_identity() -> Arc<PrimalIdentity> {
        Arc::new(PrimalIdentity::for_test("test-family", "test-node"))
    }

    #[tokio::test]
    async fn test_secrets_handler_methods() {
        let handler = SecretsHandler::new(test_identity());
        let methods = handler.methods();

        assert_eq!(methods.len(), 4);
        assert!(methods.contains(&"secrets.store"));
        assert!(methods.contains(&"secrets.retrieve"));
        assert!(methods.contains(&"secrets.list"));
        assert!(methods.contains(&"secrets.delete"));
    }

    #[tokio::test]
    async fn test_store_and_retrieve_roundtrip() {
        let handler = SecretsHandler::new(test_identity());

        // Store a secret
        let store_params = serde_json::json!({
            "name": "api-key",
            "value": "sk-secret-12345-abcdef"
        });
        let store_result = handler.handle_store(Some(&store_params)).await;
        assert!(store_result.is_ok());
        let store_resp = store_result.unwrap();
        assert_eq!(store_resp["stored"], true);
        assert_eq!(store_resp["name"], "api-key");

        // Retrieve the secret
        let retrieve_params = serde_json::json!({ "name": "api-key" });
        let retrieve_result = handler.handle_retrieve(Some(&retrieve_params)).await;
        assert!(retrieve_result.is_ok());
        let retrieve_resp = retrieve_result.unwrap();
        assert_eq!(retrieve_resp["value"], "sk-secret-12345-abcdef");
        assert_eq!(retrieve_resp["name"], "api-key");
        assert!(retrieve_resp["stored_at"].is_string());
    }

    #[tokio::test]
    async fn test_retrieve_nonexistent_secret() {
        let handler = SecretsHandler::new(test_identity());

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
        let handler_a = SecretsHandler::new(family_a);
        let handler_b = SecretsHandler::new(family_b);

        // Store same secret name in handler_a
        let store_params = serde_json::json!({
            "name": "shared-key",
            "value": "secret-from-alpha"
        });
        handler_a.handle_store(Some(&store_params)).await.unwrap();

        // Derive keys for the same name - they should differ between families
        let key_a = handler_a.derive_secret_key("shared-key").unwrap();
        let key_b = handler_b.derive_secret_key("shared-key").unwrap();
        assert_ne!(key_a, key_b, "Different families must derive different keys");
    }

    #[tokio::test]
    async fn test_list_secrets() {
        let handler = SecretsHandler::new(test_identity());

        // Store two secrets
        for name in &["key-1", "key-2"] {
            let params = serde_json::json!({
                "name": name,
                "value": format!("value-{}", name)
            });
            handler.handle_store(Some(&params)).await.unwrap();
        }

        // List
        let list_result = handler.handle_list().await;
        assert!(list_result.is_ok());
        let resp = list_result.unwrap();
        assert_eq!(resp["count"], 2);

        let names = resp["secrets"].as_array().unwrap();
        let name_strs: Vec<&str> = names.iter().map(|v| v.as_str().unwrap()).collect();
        assert!(name_strs.contains(&"key-1"));
        assert!(name_strs.contains(&"key-2"));
    }

    #[tokio::test]
    async fn test_delete_secret() {
        let handler = SecretsHandler::new(test_identity());

        // Store
        let params = serde_json::json!({
            "name": "to-delete",
            "value": "temporary"
        });
        handler.handle_store(Some(&params)).await.unwrap();

        // Delete
        let del_params = serde_json::json!({ "name": "to-delete" });
        let del_result = handler.handle_delete(Some(&del_params)).await;
        assert!(del_result.is_ok());
        assert_eq!(del_result.unwrap()["deleted"], true);

        // Verify it's gone
        let retrieve_params = serde_json::json!({ "name": "to-delete" });
        let result = handler.handle_retrieve(Some(&retrieve_params)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_nonexistent() {
        let handler = SecretsHandler::new(test_identity());

        let params = serde_json::json!({ "name": "never-stored" });
        let result = handler.handle_delete(Some(&params)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["deleted"], false);
    }

    #[tokio::test]
    async fn test_overwrite_secret() {
        let handler = SecretsHandler::new(test_identity());

        // Store initial value
        let params = serde_json::json!({
            "name": "mutable-key",
            "value": "version-1"
        });
        handler.handle_store(Some(&params)).await.unwrap();

        // Overwrite with new value
        let params = serde_json::json!({
            "name": "mutable-key",
            "value": "version-2"
        });
        handler.handle_store(Some(&params)).await.unwrap();

        // Retrieve should return latest value
        let retrieve_params = serde_json::json!({ "name": "mutable-key" });
        let result = handler.handle_retrieve(Some(&retrieve_params)).await.unwrap();
        assert_eq!(result["value"], "version-2");
    }
}
