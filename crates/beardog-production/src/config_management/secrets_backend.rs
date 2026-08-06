// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal Secrets Manager (USM) trait and `BearDog` local file vault.
//!
//! The local vault stores AES-256-GCM sealed payloads as JSON files under the configured
//! data directory. It does not speak to `HashiCorp` Vault; env names `VAULT_*` are legacy
//! compatibility for `BearDog`'s own encrypted store.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use beardog_errors::BearDogError;
use beardog_security::encryption::{EncryptionConfig, EncryptionService};
use beardog_types::constants::domains::system::defaults::DEFAULT_DATA_DIR;
use hkdf::Hkdf;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use parking_lot::Mutex;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, warn};

const VAULT_HKDF_SALT: &[u8] = b"beardog-vault-v1";
const MASTER_KEY_DOMAIN: &[u8] = b"beardog.local.vault.master.v1";
const FORMAT_VERSION: u32 = 1;

/// Pluggable secrets backend (USM). The default production implementation is
/// [`FileVaultBackend`].
pub trait SecretsBackend: Send + Sync {
    /// Persist a secret value under `key_id`.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] on invalid `key_id`, I/O, or encryption failures.
    fn store(&self, key_id: &str, value: &str) -> Result<(), BearDogError>;

    /// Load plaintext for `key_id`.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if the secret is missing, corrupt, or cannot be decrypted.
    fn retrieve(&self, key_id: &str) -> Result<String, BearDogError>;

    /// List all secret key IDs (never values).
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] on vault directory I/O failures.
    fn list(&self) -> Result<Vec<String>, BearDogError>;

    /// Delete `key_id`. Returns `true` if a file existed and was removed.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] on invalid `key_id` or I/O failures other than missing files.
    fn delete(&self, key_id: &str) -> Result<bool, BearDogError>;
}

/// On-disk record (JSON). The `sealed` field is base64(nonce || ciphertext) from
/// [`EncryptionService`] (AES-256-GCM).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VaultSecretRecord {
    format_version: u32,
    key_id: String,
    stored_at: String,
    sealed: String,
}

/// `BearDog` encrypted file vault (AES-256-GCM, per-secret HKDF-derived keys).
pub struct FileVaultBackend {
    vault_dir: PathBuf,
    master_key: [u8; 32],
    encryption: EncryptionService,
    /// Serialize mutations and scans that walk the directory.
    lock: Mutex<()>,
}

impl FileVaultBackend {
    /// Opens a vault rooted at `vault_dir` using `master_key` (32-byte AES key material).
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if the vault directory cannot be created.
    pub fn new(vault_dir: PathBuf, master_key: [u8; 32]) -> Result<Self, BearDogError> {
        fs::create_dir_all(&vault_dir).map_err(|e| {
            BearDogError::system(format!(
                "Failed to create vault directory {}: {e}",
                vault_dir.display()
            ))
        })?;
        Ok(Self {
            vault_dir,
            master_key,
            encryption: EncryptionService::new(EncryptionConfig::default()),
            lock: Mutex::new(()),
        })
    }

    /// Vault provider: `endpoint` may be a filesystem path for the vault root, or empty / HTTP(S)
    /// URL to use default data-dir layout. `token` seeds the master key unless
    /// `BEARDOG_VAULT_MASTER_KEY` (64 hex chars) is set.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if the mount path is invalid, the master key cannot be resolved, or the vault cannot be opened.
    pub fn open_for_vault_provider(
        endpoint: &str,
        token: &str,
        mount_path: &str,
    ) -> Result<Self, BearDogError> {
        let root = vault_directory_for_endpoint(endpoint);
        let root = join_mount_safe(root, mount_path)?;
        let master = resolve_master_key(Some(token), None)?;
        Self::new(root, master)
    }

    /// USM: resolve root from `endpoint` and master key from `auth_config` entries
    /// (`token`, `password`, `master_key_hex`) or `BEARDOG_VAULT_MASTER_KEY`.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if the master key cannot be resolved or the vault cannot be opened.
    pub fn open_for_usm(
        endpoint: &str,
        auth_config: &std::collections::HashMap<String, String>,
    ) -> Result<Self, BearDogError> {
        let root = vault_directory_for_endpoint(endpoint);
        let master = resolve_master_key(
            auth_config
                .get("token")
                .or_else(|| auth_config.get("password"))
                .map(String::as_str),
            auth_config.get("master_key_hex"),
        )?;
        Self::new(root, master)
    }

    fn secret_path(&self, key_id: &str) -> PathBuf {
        self.vault_dir.join(safe_filename_for_key_id(key_id))
    }

    fn derive_secret_key(&self, key_id: &str) -> Result<[u8; 32], BearDogError> {
        derive_secret_aes_key(&self.master_key, key_id)
    }
}

impl SecretsBackend for FileVaultBackend {
    fn store(&self, key_id: &str, value: &str) -> Result<(), BearDogError> {
        validate_key_id(key_id)?;
        let _g = self.lock.lock();

        let sk = self.derive_secret_key(key_id)?;
        let sealed = self.encryption.encrypt(value.as_bytes(), &sk)?;
        let sealed_b64 = B64.encode(&sealed);

        let record = VaultSecretRecord {
            format_version: FORMAT_VERSION,
            key_id: key_id.to_string(),
            stored_at: chrono::Utc::now().to_rfc3339(),
            sealed: sealed_b64,
        };
        let json = serde_json::to_vec_pretty(&record).map_err(|e| {
            BearDogError::internal(format!("Failed to serialize vault record: {e}"))
        })?;

        let path = self.secret_path(key_id);
        atomic_write(&path, &json)?;
        debug!(target: "beardog_production", key_id = %key_id, path = %path.display(), "stored secret in local vault");
        Ok(())
    }

    fn retrieve(&self, key_id: &str) -> Result<String, BearDogError> {
        validate_key_id(key_id)?;
        let _g = self.lock.lock();

        let path = self.secret_path(key_id);
        if !path.is_file() {
            return Err(BearDogError::system(format!(
                "Secret not found in vault: {key_id}"
            )));
        }

        let bytes = fs::read(&path).map_err(|e| {
            BearDogError::system(format!("Failed to read vault file {}: {e}", path.display()))
        })?;

        let record: VaultSecretRecord = serde_json::from_slice(&bytes).map_err(|e| {
            BearDogError::security(format!("Corrupt vault record for {key_id}: {e}"))
        })?;

        if record.format_version != FORMAT_VERSION {
            return Err(BearDogError::security(format!(
                "Unsupported vault format version {} for {key_id}",
                record.format_version
            )));
        }

        let sk = self.derive_secret_key(key_id)?;
        let sealed = B64.decode(record.sealed.trim()).map_err(|e| {
            BearDogError::security(format!("Invalid base64 sealed payload for {key_id}: {e}"))
        })?;

        let plain = self.encryption.decrypt(&sealed, &sk)?;
        String::from_utf8(plain).map_err(|e| {
            BearDogError::security(format!("Decrypted secret is not valid UTF-8: {e}"))
        })
    }

    fn list(&self) -> Result<Vec<String>, BearDogError> {
        let _g = self.lock.lock();

        let entries = fs::read_dir(&self.vault_dir).map_err(|e| {
            BearDogError::system(format!(
                "Failed to read vault directory {}: {e}",
                self.vault_dir.display()
            ))
        })?;

        let mut keys = Vec::new();
        for ent in entries {
            let ent =
                ent.map_err(|e| BearDogError::system(format!("Vault read_dir entry: {e}")))?;
            let path = ent.path();
            if !path
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            {
                continue;
            }
            let bytes = match fs::read(&path) {
                Ok(b) => b,
                Err(e) => {
                    warn!(target: "beardog_production", path = %path.display(), error = %e, "skipping unreadable vault file");
                    continue;
                }
            };
            if let Ok(rec) = serde_json::from_slice::<VaultSecretRecord>(&bytes)
                && rec.format_version == FORMAT_VERSION
                && !rec.key_id.is_empty()
            {
                keys.push(rec.key_id);
            }
        }
        keys.sort();
        Ok(keys)
    }

    fn delete(&self, key_id: &str) -> Result<bool, BearDogError> {
        validate_key_id(key_id)?;
        let _g = self.lock.lock();

        let path = self.secret_path(key_id);
        match fs::remove_file(&path) {
            Ok(()) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(BearDogError::system(format!(
                "Failed to delete vault secret {}: {e}",
                path.display()
            ))),
        }
    }
}

/// Default `BearDog` vault root: `$BEARDOG_DATA_DIR/vault`, else `$XDG_DATA_HOME/beardog/vault`,
/// else `{DEFAULT_DATA_DIR}/vault`, else `$HOME/.local/share/beardog/vault`.
pub fn default_vault_root() -> PathBuf {
    if let Ok(d) = beardog_errors::process_env::var("BEARDOG_DATA_DIR") {
        let p = PathBuf::from(d.trim());
        if !p.as_os_str().is_empty() {
            return p.join("vault");
        }
    }
    if let Ok(xdg) = beardog_errors::process_env::var("XDG_DATA_HOME") {
        let p = PathBuf::from(xdg.trim());
        if !p.as_os_str().is_empty() {
            return p.join("beardog").join("vault");
        }
    }
    if let Ok(home) = beardog_errors::process_env::var("HOME") {
        let p = PathBuf::from(home.trim());
        if !p.as_os_str().is_empty() {
            return p.join(".local/share/beardog/vault");
        }
    }
    PathBuf::from(DEFAULT_DATA_DIR).join("vault")
}

fn vault_directory_for_endpoint(endpoint: &str) -> PathBuf {
    let trimmed = endpoint.trim();
    if trimmed.is_empty() || trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        default_vault_root()
    } else {
        PathBuf::from(trimmed)
    }
}

fn join_mount_safe(root: PathBuf, mount_path: &str) -> Result<PathBuf, BearDogError> {
    let mp = mount_path.trim().trim_matches('/');
    if mp.is_empty() {
        return Ok(root);
    }
    if mp.contains("..") {
        return Err(BearDogError::validation(
            "vault mount_path must not contain '..'",
        ));
    }
    Ok(root.join(mp))
}

fn safe_filename_for_key_id(key_id: &str) -> String {
    let mut s = String::with_capacity(key_id.len() + 5);
    for ch in key_id.chars() {
        match ch {
            '/' => s.push_str("__"),
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' => s.push(ch),
            _ => s.push('_'),
        }
    }
    if s.is_empty() {
        s.push_str("empty_key_id");
    }
    format!("{s}.json")
}

fn validate_key_id(key_id: &str) -> Result<(), BearDogError> {
    if key_id.trim().is_empty() {
        return Err(BearDogError::validation("secret key_id must not be empty"));
    }
    Ok(())
}

fn derive_master_from_token(token: &str) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(MASTER_KEY_DOMAIN);
    h.update(token.as_bytes());
    let out = h.finalize();
    out.into()
}

fn parse_hex_master(s: &str) -> Result<[u8; 32], BearDogError> {
    let bytes = hex::decode(s.trim()).map_err(|e| {
        BearDogError::validation(&format!("BEARDOG_VAULT_MASTER_KEY / master_key_hex: {e}"))
    })?;
    if bytes.len() != 32 {
        return Err(BearDogError::validation(
            "master key material must decode to exactly 32 bytes (64 hex chars)",
        ));
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

/// Resolves 32-byte master key: env `BEARDOG_VAULT_MASTER_KEY` (hex), optional `master_key_hex`
/// from auth, else derives from `token` when present.
///
/// # Errors
///
/// Returns [`BearDogError`] if hex decoding fails or no master key source is available.
pub fn resolve_master_key(
    token: Option<&str>,
    master_key_hex: Option<&String>,
) -> Result<[u8; 32], BearDogError> {
    if let Ok(h) = beardog_errors::process_env::var("BEARDOG_VAULT_MASTER_KEY") {
        let t = h.trim();
        if !t.is_empty() {
            return parse_hex_master(t);
        }
    }
    if let Some(h) = master_key_hex {
        let t = h.trim();
        if !t.is_empty() {
            return parse_hex_master(t);
        }
    }
    let Some(tok) = token.filter(|t| !t.is_empty()) else {
        return Err(BearDogError::security(
            "Vault master key missing: set BEARDOG_VAULT_MASTER_KEY (64 hex chars) or provide a non-empty token/password"
                .to_string(),
        ));
    };
    Ok(derive_master_from_token(tok))
}

fn derive_secret_aes_key(master: &[u8; 32], key_id: &str) -> Result<[u8; 32], BearDogError> {
    let hk = Hkdf::<Sha256>::new(Some(VAULT_HKDF_SALT), master);
    let mut okm = [0u8; 32];
    hk.expand(key_id.as_bytes(), &mut okm)
        .map_err(|e| BearDogError::internal(format!("HKDF expand failed: {e}")))?;
    Ok(okm)
}

fn atomic_write(path: &Path, data: &[u8]) -> Result<(), BearDogError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            BearDogError::system(format!("Failed to create parent {}: {e}", parent.display()))
        })?;
    }
    let tmp = path.with_extension("json.part");
    fs::write(&tmp, data).map_err(|e| {
        BearDogError::system(format!(
            "Failed to write temp vault file {}: {e}",
            tmp.display()
        ))
    })?;
    fs::rename(&tmp, path).map_err(|e| {
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
    use tempfile::tempdir;

    #[test]
    fn vault_roundtrip_store_retrieve_list_delete() {
        let dir = tempdir().expect("tempdir");
        let master = [7u8; 32];
        let v = FileVaultBackend::new(dir.path().join("vault"), master).expect("new vault");

        v.store("database/password", "hunter2").expect("store");
        assert_eq!(v.retrieve("database/password").expect("get"), "hunter2");

        let keys = v.list().expect("list");
        assert!(keys.contains(&"database/password".to_string()));

        assert!(v.delete("database/password").expect("del"));
        assert!(!v.delete("database/password").expect("del again"));
        assert!(v.retrieve("database/password").is_err());
    }
}
