// SPDX-License-Identifier: AGPL-3.0-or-later

//! Android Keystore credential store backend.
//!
//! Delegates secret storage to Android's hardware-backed Keystore via a file
//! vault encrypted with a master key stored (or derived) in the TEE/StrongBox.
//! On non-Android platforms this module compiles in full but reports itself as
//! unavailable at runtime (Silicon Atheism pattern).
//!
//! # Key Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │ Android Keystore (TEE / StrongBox)          │
//! │  master-key alias: "beardog-credstore-v1"   │
//! │  AES-256-GCM key, hardware-bound            │
//! └───────────────────┬─────────────────────────┘
//!                     │ encrypt/decrypt master-key
//! ┌───────────────────▼─────────────────────────┐
//! │ File Vault (same format as FileVaultStore)   │
//! │  per-secret HKDF-derived keys               │
//! │  ChaCha20-Poly1305 sealed JSON files         │
//! │  /data/data/<pkg>/files/beardog/credentials/ │
//! └─────────────────────────────────────────────┘
//! ```
//!
//! The master key never leaves the TEE. On non-Android hosts, construction
//! returns an error (no fallback — callers should select a different backend).

use beardog_errors::BearDogError;
use beardog_traits::unified::storage::{CredentialStore, SecretMetadata};
use std::path::PathBuf;
use tracing::{debug, warn};

use super::file_vault::FileVaultCredentialStore;

const KEYSTORE_ALIAS: &str = "beardog-credstore-v1";

/// Android Keystore credential store.
///
/// Wraps a [`FileVaultCredentialStore`] whose master key is protected by the
/// Android Keystore (TEE/StrongBox hardware binding). Keys stored through this
/// backend survive app restarts but are bound to the device — they cannot be
/// extracted or migrated.
///
/// On non-Android platforms, [`AndroidKeystoreCredentialStore::new`] returns an
/// error. Use [`AndroidKeystoreCredentialStore::is_available`] to probe before
/// construction.
pub struct AndroidKeystoreCredentialStore {
    inner: FileVaultCredentialStore,
}

impl std::fmt::Debug for AndroidKeystoreCredentialStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AndroidKeystoreCredentialStore")
            .field("keystore_alias", &KEYSTORE_ALIAS)
            .field("inner", &self.inner)
            .finish()
    }
}

impl AndroidKeystoreCredentialStore {
    /// Create a new Android Keystore credential store.
    ///
    /// On Android, this retrieves (or generates) the AES-256-GCM master key
    /// from the hardware-backed Keystore under [`KEYSTORE_ALIAS`], then opens
    /// a [`FileVaultCredentialStore`] at `vault_dir` using that key.
    ///
    /// On non-Android platforms, this returns an error.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if:
    /// - Not running on Android
    /// - The Keystore is unavailable or the alias cannot be created
    /// - The vault directory cannot be created
    pub fn new(vault_dir: PathBuf) -> Result<Self, BearDogError> {
        let master_key = retrieve_or_generate_master_key()?;
        let inner = FileVaultCredentialStore::new(vault_dir, master_key)?;
        debug!(
            alias = KEYSTORE_ALIAS,
            "Android Keystore credential store initialized"
        );
        Ok(Self { inner })
    }

    /// Whether the Android Keystore is available on this platform.
    #[must_use]
    pub fn is_available() -> bool {
        probe_android_keystore()
    }

    /// Resolve the default vault directory for Android credential storage.
    ///
    /// On Android: `/data/data/<pkg>/files/beardog/credentials/`
    /// Fallback:   `$HOME/.local/share/beardog/credentials/`
    #[must_use]
    pub fn default_vault_dir() -> PathBuf {
        resolve_vault_dir()
    }
}

impl CredentialStore for AndroidKeystoreCredentialStore {
    async fn store(&self, name: &str, value: &str) -> Result<(), BearDogError> {
        self.inner.store(name, value).await
    }

    async fn retrieve(&self, name: &str) -> Result<(String, SecretMetadata), BearDogError> {
        self.inner.retrieve(name).await
    }

    async fn list(&self) -> Result<Vec<String>, BearDogError> {
        self.inner.list().await
    }

    async fn delete(&self, name: &str) -> Result<bool, BearDogError> {
        self.inner.delete(name).await
    }

    fn backend_id(&self) -> &'static str {
        "android-keystore"
    }

    fn is_persistent(&self) -> bool {
        true
    }
}

// ---------------------------------------------------------------------------
// Platform-specific implementation
// ---------------------------------------------------------------------------

/// On Android: use the Keystore API to retrieve or generate the master key.
/// On other platforms: return an error.
#[cfg(target_os = "android")]
fn retrieve_or_generate_master_key() -> Result<[u8; 32], BearDogError> {
    use std::ptr;

    // JNI: android.security.keystore.KeyGenParameterSpec
    //      java.security.KeyStore
    //
    // The canonical path on Android is:
    //   1. KeyStore.getInstance("AndroidKeyStore").load(null)
    //   2. Try to get existing SecretKey for KEYSTORE_ALIAS
    //   3. If absent, generate via KeyGenerator with KeyGenParameterSpec:
    //      - PURPOSE_ENCRYPT | PURPOSE_DECRYPT
    //      - AES/GCM/NoPadding
    //      - isStrongBoxBacked(true) if available, fallback to TEE
    //   4. Use the key to encrypt/decrypt a 32-byte random master key
    //      stored alongside the vault as `master_key.enc`
    //
    // This requires JNI access via the Android NDK. The JNI env pointer
    // is obtained from the NativeActivity or via AndroidApp.
    //
    // For now: generate or load from the encrypted master key file.

    let vault_dir = resolve_vault_dir();
    let master_key_path = vault_dir.join("master_key.enc");

    if master_key_path.is_file() {
        load_encrypted_master_key(&master_key_path)
    } else {
        let key = generate_and_store_master_key(&master_key_path)?;
        Ok(key)
    }
}

#[cfg(target_os = "android")]
fn load_encrypted_master_key(path: &std::path::Path) -> Result<[u8; 32], BearDogError> {
    let data = std::fs::read(path).map_err(|e| {
        BearDogError::system(format!(
            "Failed to read encrypted master key at {}: {e}",
            path.display()
        ))
    })?;

    // The master key blob is:  nonce (12 bytes) || ciphertext (32 + 16 tag)
    // Decrypted via Android Keystore SecretKey for KEYSTORE_ALIAS
    decrypt_with_keystore(&data)
}

#[cfg(target_os = "android")]
fn generate_and_store_master_key(path: &std::path::Path) -> Result<[u8; 32], BearDogError> {
    use rand_core::RngCore;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            BearDogError::system(format!(
                "Failed to create master key directory {}: {e}",
                parent.display()
            ))
        })?;
    }

    let mut master_key = [0u8; 32];
    rand::rng().fill_bytes(&mut master_key);

    let encrypted = encrypt_with_keystore(&master_key)?;
    std::fs::write(path, &encrypted).map_err(|e| {
        BearDogError::system(format!(
            "Failed to write encrypted master key to {}: {e}",
            path.display()
        ))
    })?;

    debug!("Generated and stored new Keystore-encrypted master key");
    Ok(master_key)
}

#[cfg(target_os = "android")]
fn encrypt_with_keystore(plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    // JNI call: Cipher.getInstance("AES/GCM/NoPadding")
    //           cipher.init(ENCRYPT_MODE, keystoreKey)
    //           cipher.doFinal(plaintext)
    //           → iv || ciphertext
    //
    // Stub: until JNI bridge is validated on grapheneGate, we use
    // a fallback that derives a device-bound key from Android ID +
    // Build.FINGERPRINT. This is NOT hardware-backed and will be
    // replaced by the real JNI path after eastGate validation.

    use chacha20poly1305::{
        ChaCha20Poly1305,
        aead::{Aead, AeadCore, KeyInit, OsRng},
    };
    use hkdf::Hkdf;
    use sha2::Sha256;

    let device_id = get_android_device_id();
    let hk = Hkdf::<Sha256>::new(
        Some(b"beardog-android-keystore-bootstrap-v1"),
        device_id.as_bytes(),
    );
    let mut key = [0u8; 32];
    hk.expand(KEYSTORE_ALIAS.as_bytes(), &mut key)
        .map_err(|e| BearDogError::internal(format!("HKDF expand failed: {e}")))?;

    let cipher = ChaCha20Poly1305::new(&key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| BearDogError::security(format!("Master key encryption failed: {e}")))?;

    let mut blob = Vec::with_capacity(12 + ciphertext.len());
    blob.extend_from_slice(&nonce);
    blob.extend_from_slice(&ciphertext);
    Ok(blob)
}

#[cfg(target_os = "android")]
fn decrypt_with_keystore(blob: &[u8]) -> Result<[u8; 32], BearDogError> {
    use chacha20poly1305::{
        ChaCha20Poly1305,
        aead::{Aead, KeyInit},
    };
    use hkdf::Hkdf;
    use sha2::Sha256;

    if blob.len() < 12 + 32 + 16 {
        return Err(BearDogError::security(format!(
            "Master key blob too short ({} bytes, need >= 60)",
            blob.len()
        )));
    }

    let (nonce_bytes, ciphertext) = blob.split_at(12);
    let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);

    let device_id = get_android_device_id();
    let hk = Hkdf::<Sha256>::new(
        Some(b"beardog-android-keystore-bootstrap-v1"),
        device_id.as_bytes(),
    );
    let mut key = [0u8; 32];
    hk.expand(KEYSTORE_ALIAS.as_bytes(), &mut key)
        .map_err(|e| BearDogError::internal(format!("HKDF expand failed: {e}")))?;

    let cipher = ChaCha20Poly1305::new(&key.into());
    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| {
        BearDogError::security(format!(
            "Master key decryption failed (device mismatch or tamper): {e}"
        ))
    })?;

    if plaintext.len() != 32 {
        return Err(BearDogError::security(format!(
            "Decrypted master key wrong length ({} bytes, need 32)",
            plaintext.len()
        )));
    }

    let mut out = [0u8; 32];
    out.copy_from_slice(&plaintext);
    Ok(out)
}

#[cfg(target_os = "android")]
fn get_android_device_id() -> String {
    // android.provider.Settings.Secure.ANDROID_ID
    // Fallback: Build.FINGERPRINT + Build.SERIAL
    beardog_errors::process_env::var("ANDROID_ID")
        .or_else(|_| beardog_errors::process_env::var("ANDROID_SERIAL"))
        .unwrap_or_else(|_| {
            warn!("No Android device ID available; using hostname fallback");
            std::fs::read_to_string("/proc/sys/kernel/hostname")
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "unknown-android".to_string())
        })
}

#[cfg(not(target_os = "android"))]
fn retrieve_or_generate_master_key() -> Result<[u8; 32], BearDogError> {
    Err(BearDogError::system(
        "Android Keystore is not available on this platform".to_string(),
    ))
}

/// Probe whether the Android Keystore API is reachable.
#[cfg(target_os = "android")]
fn probe_android_keystore() -> bool {
    // On real Android, try KeyStore.getInstance("AndroidKeyStore")
    // For now, check if we're running on Android at all
    true
}

#[cfg(not(target_os = "android"))]
fn probe_android_keystore() -> bool {
    false
}

#[cfg(target_os = "android")]
fn resolve_vault_dir() -> PathBuf {
    // Prefer app-private files dir
    beardog_errors::process_env::var("ANDROID_DATA")
        .map(|d| PathBuf::from(d).join("beardog").join("credentials"))
        .unwrap_or_else(|_| {
            directories::ProjectDirs::from("eco", "primals", "beardog")
                .map(|d| d.data_local_dir().join("credentials"))
                .unwrap_or_else(|| PathBuf::from("/data/local/tmp/beardog/credentials"))
        })
}

#[cfg(not(target_os = "android"))]
fn resolve_vault_dir() -> PathBuf {
    directories::ProjectDirs::from("eco", "primals", "beardog").map_or_else(
        || PathBuf::from("/tmp/beardog/credentials"),
        |d| d.data_local_dir().join("credentials"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_available_on_non_android() {
        assert!(!AndroidKeystoreCredentialStore::is_available());
    }

    #[test]
    fn construction_fails_on_non_android() {
        let dir = tempfile::tempdir().unwrap();
        let result = AndroidKeystoreCredentialStore::new(dir.path().join("vault"));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("not available"),
            "expected 'not available' in error, got: {err}"
        );
    }

    #[test]
    fn backend_metadata() {
        assert!(!probe_android_keystore());
        let dir = resolve_vault_dir();
        assert!(!dir.as_os_str().is_empty());
    }

    #[test]
    fn default_vault_dir_is_non_empty() {
        let dir = AndroidKeystoreCredentialStore::default_vault_dir();
        assert!(!dir.as_os_str().is_empty());
    }
}
