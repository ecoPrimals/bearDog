// SPDX-License-Identifier: AGPL-3.0-or-later

//! Encrypted file vault credential store backend.
//!
//! Each secret is stored as a ChaCha20-Poly1305 sealed JSON file under a
//! configurable vault directory. Encryption keys are derived per-secret via
//! HKDF-SHA256 from a master key, providing cryptographic isolation between
//! entries.

use beardog_errors::BearDogError;
use beardog_traits::unified::storage::{CredentialStore, SecretMetadata};
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use hkdf::Hkdf;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::path::{Path, PathBuf};
use tracing::debug;

const VAULT_HKDF_SALT: &[u8] = b"beardog-credential-store-v1";
const FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VaultRecord {
    format_version: u32,
    name: String,
    stored_at: String,
    nonce_hex: String,
    ciphertext_hex: String,
}

/// Persistent encrypted file vault credential store.
///
/// Secrets are stored as individual JSON files under `vault_dir`, each sealed
/// with ChaCha20-Poly1305 using a per-secret HKDF-derived key. The master key
/// (32 bytes) must be supplied at construction.
pub struct FileVaultCredentialStore {
    vault_dir: PathBuf,
    master_key: [u8; 32],
    lock: Mutex<()>,
}

impl std::fmt::Debug for FileVaultCredentialStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileVaultCredentialStore")
            .field("vault_dir", &self.vault_dir)
            .field("master_key", &"[REDACTED]")
            .finish()
    }
}

impl FileVaultCredentialStore {
    /// Open (or create) a file vault at `vault_dir` with the given master key.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if the directory cannot be created.
    pub fn new(vault_dir: PathBuf, master_key: [u8; 32]) -> Result<Self, BearDogError> {
        std::fs::create_dir_all(&vault_dir).map_err(|e| {
            BearDogError::system(format!(
                "Failed to create credential vault directory {}: {e}",
                vault_dir.display()
            ))
        })?;
        Ok(Self {
            vault_dir,
            master_key,
            lock: Mutex::new(()),
        })
    }

    fn derive_key(&self, name: &str) -> Result<[u8; 32], BearDogError> {
        let hk = Hkdf::<Sha256>::new(Some(VAULT_HKDF_SALT), &self.master_key);
        let mut okm = [0u8; 32];
        hk.expand(name.as_bytes(), &mut okm)
            .map_err(|e| BearDogError::internal(format!("HKDF expand failed: {e}")))?;
        Ok(okm)
    }

    fn record_path(&self, name: &str) -> PathBuf {
        self.vault_dir.join(safe_filename(name))
    }
}

impl CredentialStore for FileVaultCredentialStore {
    async fn store(&self, name: &str, value: &str) -> Result<(), BearDogError> {
        validate_name(name)?;
        let _g = self.lock.lock();

        let key = self.derive_key(name)?;
        let cipher = ChaCha20Poly1305::new(&key.into());
        let nonce = ChaCha20Poly1305::generate_nonce(OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, value.as_bytes())
            .map_err(|e| BearDogError::internal(format!("Encryption failed: {e}")))?;

        let record = VaultRecord {
            format_version: FORMAT_VERSION,
            name: name.to_string(),
            stored_at: chrono::Utc::now().to_rfc3339(),
            nonce_hex: hex::encode(nonce),
            ciphertext_hex: hex::encode(ciphertext),
        };

        let json = serde_json::to_vec_pretty(&record).map_err(|e| {
            BearDogError::internal(format!("Failed to serialize vault record: {e}"))
        })?;

        let path = self.record_path(name);
        atomic_write(&path, &json)?;
        debug!(name, path = %path.display(), "credential stored in file vault");
        Ok(())
    }

    async fn retrieve(&self, name: &str) -> Result<(String, SecretMetadata), BearDogError> {
        validate_name(name)?;
        let _g = self.lock.lock();

        let path = self.record_path(name);
        if !path.is_file() {
            return Err(BearDogError::system(format!(
                "Credential '{name}' not found in vault"
            )));
        }

        let bytes = std::fs::read(&path).map_err(|e| {
            BearDogError::system(format!("Failed to read vault file {}: {e}", path.display()))
        })?;

        let record: VaultRecord = serde_json::from_slice(&bytes).map_err(|e| {
            BearDogError::security(format!("Corrupt vault record for '{name}': {e}"))
        })?;

        if record.format_version != FORMAT_VERSION {
            return Err(BearDogError::security(format!(
                "Unsupported vault format version {} for '{name}'",
                record.format_version
            )));
        }

        let key = self.derive_key(name)?;
        let cipher = ChaCha20Poly1305::new(&key.into());
        let nonce_bytes = hex::decode(&record.nonce_hex)
            .map_err(|e| BearDogError::security(format!("Corrupt nonce hex for '{name}': {e}")))?;

        if nonce_bytes.len() != 12 {
            return Err(BearDogError::security(format!(
                "Invalid nonce length {} for '{name}'",
                nonce_bytes.len()
            )));
        }

        let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);
        let ciphertext = hex::decode(&record.ciphertext_hex).map_err(|e| {
            BearDogError::security(format!("Corrupt ciphertext hex for '{name}': {e}"))
        })?;

        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).map_err(|e| {
            BearDogError::security(format!(
                "Decryption failed for '{name}' (tampering or key mismatch): {e}"
            ))
        })?;

        let value = String::from_utf8(plaintext).map_err(|e| {
            BearDogError::security(format!("Decrypted value for '{name}' is not UTF-8: {e}"))
        })?;

        Ok((
            value,
            SecretMetadata {
                stored_at: record.stored_at,
            },
        ))
    }

    async fn list(&self) -> Result<Vec<String>, BearDogError> {
        let _g = self.lock.lock();

        let entries = std::fs::read_dir(&self.vault_dir).map_err(|e| {
            BearDogError::system(format!(
                "Failed to read vault directory {}: {e}",
                self.vault_dir.display()
            ))
        })?;

        let mut names = Vec::new();
        for entry in entries {
            let entry =
                entry.map_err(|e| BearDogError::system(format!("Vault read_dir entry: {e}")))?;
            let path = entry.path();
            if !path
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            {
                continue;
            }
            let Ok(bytes) = std::fs::read(&path) else {
                continue;
            };
            if let Ok(rec) = serde_json::from_slice::<VaultRecord>(&bytes)
                && rec.format_version == FORMAT_VERSION
                && !rec.name.is_empty()
            {
                names.push(rec.name);
            }
        }
        names.sort();
        Ok(names)
    }

    async fn delete(&self, name: &str) -> Result<bool, BearDogError> {
        validate_name(name)?;
        let _g = self.lock.lock();

        let path = self.record_path(name);
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(BearDogError::system(format!(
                "Failed to delete vault credential {}: {e}",
                path.display()
            ))),
        }
    }

    fn backend_id(&self) -> &'static str {
        "file-vault"
    }

    fn is_persistent(&self) -> bool {
        true
    }
}

fn validate_name(name: &str) -> Result<(), BearDogError> {
    if name.trim().is_empty() {
        return Err(BearDogError::validation(
            "credential name must not be empty",
        ));
    }
    Ok(())
}

fn safe_filename(name: &str) -> String {
    let mut s = String::with_capacity(name.len() + 5);
    for ch in name.chars() {
        match ch {
            '/' | ':' => s.push_str("__"),
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' => s.push(ch),
            _ => s.push('_'),
        }
    }
    if s.is_empty() {
        s.push_str("_empty_");
    }
    format!("{s}.json")
}

fn atomic_write(path: &Path, data: &[u8]) -> Result<(), BearDogError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            BearDogError::system(format!(
                "Failed to create parent directory {}: {e}",
                parent.display()
            ))
        })?;
    }
    let tmp = path.with_extension("json.part");
    std::fs::write(&tmp, data).map_err(|e| {
        BearDogError::system(format!(
            "Failed to write temp vault file {}: {e}",
            tmp.display()
        ))
    })?;
    std::fs::rename(&tmp, path).map_err(|e| {
        BearDogError::system(format!(
            "Failed to finalize vault file {}: {e}",
            path.display()
        ))
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn roundtrip_store_retrieve_delete() {
        let dir = tempfile::tempdir().unwrap();
        let vault = FileVaultCredentialStore::new(dir.path().join("vault"), [42u8; 32]).unwrap();

        vault.store("database/password", "hunter2").await.unwrap();

        let (val, meta) = vault.retrieve("database/password").await.unwrap();
        assert_eq!(val, "hunter2");
        assert!(!meta.stored_at.is_empty());

        let names = vault.list().await.unwrap();
        assert!(names.contains(&"database/password".to_string()));

        assert!(vault.delete("database/password").await.unwrap());
        assert!(!vault.delete("database/password").await.unwrap());
        assert!(vault.retrieve("database/password").await.is_err());
    }

    #[tokio::test]
    async fn overwrite_replaces_value() {
        let dir = tempfile::tempdir().unwrap();
        let vault = FileVaultCredentialStore::new(dir.path().join("vault"), [7u8; 32]).unwrap();

        vault.store("k", "v1").await.unwrap();
        vault.store("k", "v2").await.unwrap();

        let (val, _) = vault.retrieve("k").await.unwrap();
        assert_eq!(val, "v2");
    }

    #[tokio::test]
    async fn empty_name_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let vault = FileVaultCredentialStore::new(dir.path().join("vault"), [0u8; 32]).unwrap();
        assert!(vault.store("", "val").await.is_err());
    }

    #[test]
    fn backend_identity() {
        let dir = tempfile::tempdir().unwrap();
        let vault = FileVaultCredentialStore::new(dir.path().join("vault"), [1u8; 32]).unwrap();
        assert_eq!(vault.backend_id(), "file-vault");
        assert!(vault.is_persistent());
    }
}
