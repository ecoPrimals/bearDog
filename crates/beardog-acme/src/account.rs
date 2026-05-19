// SPDX-License-Identifier: AGPL-3.0-or-later

//! ACME account management — key generation, registration, and persistence.

use crate::error::AcmeError;
use crate::jws;
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

/// Persistent ACME account representation.
#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeAccount {
    /// Ed25519 signing key bytes (32 bytes, secret).
    #[serde(with = "hex_key")]
    secret_key: [u8; 32],

    /// Account URL returned by the ACME server after registration.
    pub account_url: Option<String>,

    /// Contact email addresses registered with the CA.
    pub contacts: Vec<String>,
}

impl AcmeAccount {
    /// Generate a new ACME account with a fresh Ed25519 keypair.
    #[must_use]
    pub fn generate(contacts: Vec<String>) -> Self {
        let secret: [u8; 32] = rand::random();
        let signing_key = SigningKey::from_bytes(&secret);
        Self {
            secret_key: signing_key.to_bytes(),
            account_url: None,
            contacts,
        }
    }

    /// Load an existing account from a JSON file, or generate a new one.
    ///
    /// # Errors
    ///
    /// Returns an error if the file exists but cannot be read or parsed.
    pub fn load_or_create(path: &Path, contacts: Vec<String>) -> Result<Self, AcmeError> {
        if path.exists() {
            let data = std::fs::read_to_string(path)?;
            let account: Self = serde_json::from_str(&data)?;
            info!(account_url = ?account.account_url, "loaded existing ACME account");
            Ok(account)
        } else {
            let account = Self::generate(contacts);
            info!("generated new ACME account keypair");
            Ok(account)
        }
    }

    /// Persist the account to a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save(&self, path: &Path) -> Result<(), AcmeError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    /// Get the Ed25519 signing key for this account.
    #[must_use]
    pub fn signing_key(&self) -> SigningKey {
        SigningKey::from_bytes(&self.secret_key)
    }

    /// Get the JWK thumbprint of this account's public key.
    #[must_use]
    pub fn thumbprint(&self) -> String {
        jws::jwk_thumbprint(&self.signing_key().verifying_key())
    }

    /// Set the account URL after successful registration.
    pub fn set_account_url(&mut self, url: String) {
        self.account_url = Some(url);
    }

    /// Whether this account has been registered with the CA.
    #[must_use]
    pub fn is_registered(&self) -> bool {
        self.account_url.is_some()
    }
}

/// Hex serialization for the 32-byte secret key.
mod hex_key {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(key: &[u8; 32], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&hex::encode(key))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
        let s = String::deserialize(d)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("expected 32 bytes"))?;
        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn generate_creates_valid_account() {
        let account = AcmeAccount::generate(vec!["mailto:test@example.com".to_string()]);
        assert!(!account.is_registered());
        assert_eq!(account.contacts.len(), 1);
        assert!(!account.thumbprint().is_empty());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("account.json");

        let mut account = AcmeAccount::generate(vec!["mailto:a@b.com".to_string()]);
        account.set_account_url("https://acme.example/acct/1".to_string());
        account.save(&path).expect("save");

        let loaded = AcmeAccount::load_or_create(&path, vec![]).expect("load");
        assert_eq!(loaded.account_url, account.account_url);
        assert_eq!(loaded.thumbprint(), account.thumbprint());
    }

    #[test]
    fn load_or_create_generates_when_missing() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("nonexistent.json");

        let account =
            AcmeAccount::load_or_create(&path, vec!["mailto:x@y.com".to_string()]).expect("create");
        assert!(!account.is_registered());
    }

    #[test]
    fn signing_key_is_deterministic() {
        let account = AcmeAccount::generate(vec![]);
        let k1 = account.signing_key();
        let k2 = account.signing_key();
        assert_eq!(k1.to_bytes(), k2.to_bytes());
    }
}
