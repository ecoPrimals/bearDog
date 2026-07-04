// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::Engine;
use beardog_config::env_keys::ENV_TRUSTED_ISSUERS;
use beardog_errors::BearDogError;
use ed25519_dalek::VerifyingKey;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::did::{did_from_verifying_key, did_matches_key};
use super::types::{IssuerInfo, RegisterError, TrustMethod};

/// Thread-safe registry of trusted remote issuers.
///
/// Keyed by issuer DID string, stores the `VerifyingKey` and metadata.
#[derive(Debug, Clone)]
pub struct TrustedIssuerRegistry {
    pub(crate) inner: Arc<RwLock<RegistryInner>>,
}

#[derive(Debug)]
pub struct RegistryInner {
    pub(crate) issuers: HashMap<String, (VerifyingKey, IssuerInfo)>,
}

impl TrustedIssuerRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(RegistryInner {
                issuers: HashMap::new(),
            })),
        }
    }

    /// Register a trusted remote issuer.
    ///
    /// The `did` must be the canonical `did:key:z6Mk...` derived from the
    /// supplied `key` — the registry validates this binding to prevent
    /// mismatched DID/key registration.
    ///
    /// Returns `Ok(true)` if newly registered, `Ok(false)` if already
    /// present, or `Err` if the DID does not match the key.
    ///
    /// # Errors
    ///
    /// Returns [`RegisterError::DidKeyMismatch`] if the supplied DID does
    /// not match the canonical DID derived from the public key.
    pub fn register(
        &self,
        did: &str,
        key: VerifyingKey,
        gate_id: Option<String>,
        family_id: Option<String>,
        method: TrustMethod,
    ) -> Result<bool, RegisterError> {
        if !did_matches_key(did, &key) {
            let expected = did_from_verifying_key(&key);
            return Err(RegisterError::DidKeyMismatch { expected });
        }

        let mut inner = self
            .inner
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if inner.issuers.contains_key(did) {
            return Ok(false);
        }
        let info = IssuerInfo {
            did: did.to_owned(),
            gate_id,
            family_id,
            registered_at: chrono::Utc::now().timestamp(),
            trust_method: method,
        };
        inner.issuers.insert(did.to_owned(), (key, info));
        Ok(true)
    }

    /// Look up a trusted issuer by DID.
    #[must_use]
    pub fn get(&self, did: &str) -> Option<(VerifyingKey, IssuerInfo)> {
        let inner = self
            .inner
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.issuers.get(did).cloned()
    }

    /// Number of registered issuers.
    #[must_use]
    pub fn len(&self) -> usize {
        let inner = self
            .inner
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.issuers.len()
    }

    /// Whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// List all registered issuer DIDs with metadata (for `auth.trusted_issuers`).
    #[must_use]
    pub fn list(&self) -> Vec<IssuerInfo> {
        let inner = self
            .inner
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        inner
            .issuers
            .values()
            .map(|(_, info)| info.clone())
            .collect()
    }

    /// Parse and seed issuers from [`ENV_TRUSTED_ISSUERS`].
    ///
    /// Each comma-separated entry uses `public_key_base64:gate_id:family_id`
    /// (`gate_id` and `family_id` are optional). The DID is derived from the key.
    ///
    /// # Errors
    ///
    /// Returns an error if any entry contains invalid base64 or an invalid Ed25519 key.
    pub fn seed_from_env(&self) -> Result<usize, BearDogError> {
        let Ok(raw) = beardog_errors::process_env::var(ENV_TRUSTED_ISSUERS) else {
            return Ok(0);
        };
        let mut seeded = 0usize;
        for entry in raw.split(',') {
            let entry = entry.trim();
            if entry.is_empty() {
                continue;
            }
            let parts: Vec<&str> = entry.splitn(3, ':').collect();
            if parts.is_empty() {
                continue;
            }
            let key_b64 = parts[0].trim();
            let gate_id = parts.get(1).map(|s| s.trim().to_string());
            let family_id = parts.get(2).map(|s| s.trim().to_string());

            let key_bytes = base64::engine::general_purpose::STANDARD
                .decode(key_b64)
                .map_err(|e| {
                    BearDogError::invalid_input(&format!(
                        "Invalid base64 in BEARDOG_TRUSTED_ISSUERS: {e}"
                    ))
                })?;
            let key_array: [u8; 32] = key_bytes.try_into().map_err(|_| {
                BearDogError::invalid_input("BEARDOG_TRUSTED_ISSUERS key not 32 bytes")
            })?;
            let vk = VerifyingKey::from_bytes(&key_array).map_err(|e| {
                BearDogError::invalid_input(&format!(
                    "Invalid Ed25519 key in BEARDOG_TRUSTED_ISSUERS: {e}"
                ))
            })?;

            let did = did_from_verifying_key(&vk);
            if self
                .register(&did, vk, gate_id, family_id, TrustMethod::Manual)
                .map_err(|e| BearDogError::invalid_input(&e.to_string()))?
            {
                seeded += 1;
                tracing::info!(did = %did, "Seeded trusted issuer from env");
            }
        }
        Ok(seeded)
    }

    /// Remove a trusted issuer by DID. Returns `true` if removed.
    pub fn remove(&self, did: &str) -> bool {
        let mut inner = self
            .inner
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        inner.issuers.remove(did).is_some()
    }
}

impl Default for TrustedIssuerRegistry {
    fn default() -> Self {
        Self::new()
    }
}
