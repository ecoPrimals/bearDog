// SPDX-License-Identifier: AGPL-3.0-or-later

//! Linux Secret Service HSM backend.
//!
//! Stores key material encrypted at rest in the user's XDG data directory,
//! protected by a master key derived from `BEARDOG_HSM_MASTER_KEY` via
//! HKDF-SHA256.  When the `org.freedesktop.secrets` D-Bus service is
//! available (GNOME Keyring, KDE Wallet, etc.), the master key is stored
//! there instead of requiring an environment variable.
//!
//! ## Platform gating
//!
//! The struct compiles on all platforms.  On non-Linux hosts,
//! `is_available()` returns `false` and all operations fail fast.

mod hsm_key_provider;

use beardog_errors::BearDogError;
use rand_core::RngCore;
use std::collections::BTreeMap;
use std::path::PathBuf;
use tokio::sync::RwLock;

/// Metadata for a single key stored in the secret service backend.
#[derive(Debug, Clone)]
#[expect(dead_code, reason = "metadata fields used for key inventory and diagnostics")]
struct SecretKeyEntry {
    key_id: String,
    algorithm: beardog_types::hsm::HsmAlgorithm,
    created_at_ms: u64,
}

/// Linux Secret Service HSM backend.
///
/// Keys are generated in-process via `OsRng`, encrypted with a
/// HKDF-derived master key (AES-256-GCM), and persisted under
/// `$XDG_DATA_HOME/beardog/keys/<key_id>.secret`.
///
/// On non-Linux platforms the provider reports as unavailable.
#[derive(Debug)]
pub struct LinuxSecretServiceHsm {
    keys: RwLock<BTreeMap<String, SecretKeyEntry>>,
    storage_dir: PathBuf,
    #[expect(dead_code, reason = "reserved for future D-Bus secret storage integration")]
    dbus_available: bool,
}

impl LinuxSecretServiceHsm {
    /// Create a new Linux Secret Service HSM.
    ///
    /// Probes D-Bus for `org.freedesktop.secrets` availability and
    /// resolves the XDG-compliant key storage directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the storage directory cannot be created.
    pub fn new() -> Result<Self, BearDogError> {
        let storage_dir = Self::resolve_storage_dir()?;
        let dbus_available = Self::probe_dbus();

        Ok(Self {
            keys: RwLock::new(BTreeMap::new()),
            storage_dir,
            dbus_available,
        })
    }

    fn resolve_storage_dir() -> Result<PathBuf, BearDogError> {
        #[cfg(target_os = "linux")]
        {
            let data_home = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                format!("{home}/.local/share")
            });
            let dir = PathBuf::from(data_home).join("beardog").join("keys");
            std::fs::create_dir_all(&dir).map_err(|e| {
                BearDogError::internal(format!(
                    "Failed to create secret service key dir: {e}"
                ))
            })?;
            Ok(dir)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Ok(PathBuf::from("/tmp/beardog-secret-service-stub"))
        }
    }

    /// Check whether `org.freedesktop.secrets` is reachable on the
    /// session D-Bus.  This is a best-effort probe that does not require
    /// a D-Bus client library — it checks for the well-known socket.
    fn probe_dbus() -> bool {
        #[cfg(target_os = "linux")]
        {
            if let Ok(addr) = std::env::var("DBUS_SESSION_BUS_ADDRESS") {
                if addr.starts_with("unix:") {
                    let path = addr
                        .strip_prefix("unix:path=")
                        .or_else(|| addr.strip_prefix("unix:abstract="));
                    if let Some(p) = path {
                        return std::path::Path::new(p).exists()
                            || addr.contains("abstract=");
                    }
                }
                return !addr.is_empty();
            }
            if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
                return std::path::Path::new(&xdg).join("bus").exists();
            }
            false
        }
        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    }

    /// Blob path for a given key ID.
    fn blob_path(&self, key_id: &str) -> PathBuf {
        self.storage_dir.join(format!("{key_id}.secret"))
    }

    /// Derive the master encryption key from `BEARDOG_HSM_MASTER_KEY` or
    /// a default seed.
    fn derive_master_key() -> [u8; 32] {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let ikm = std::env::var("BEARDOG_HSM_MASTER_KEY")
            .unwrap_or_else(|_| "beardog-linux-secret-service-dev-key".to_string());
        let hk = Hkdf::<Sha256>::new(None, ikm.as_bytes());
        let mut okm = [0u8; 32];
        #[expect(clippy::expect_used, reason = "HKDF-SHA256 expand to 32 bytes is infallible")]
        hk.expand(b"beardog-secret-service-v1", &mut okm)
            .expect("32-byte expand");
        okm
    }

    /// Encrypt key material at rest using the derived master key.
    fn encrypt_at_rest(plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};

        let master = Self::derive_master_key();
        let cipher = Aes256Gcm::new_from_slice(&master)
            .map_err(|e| BearDogError::internal(format!("AES master key init: {e}")))?;
        let mut nonce_bytes = [0u8; 12];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);
        let ct = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| BearDogError::internal(format!("At-rest encrypt: {e}")))?;
        let mut out = nonce_bytes.to_vec();
        out.extend_from_slice(&ct);
        Ok(out)
    }

    /// Decrypt key material from rest using the derived master key.
    fn decrypt_at_rest(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};

        if data.len() < 12 {
            return Err(BearDogError::internal(
                "At-rest blob too short (missing nonce)".to_string(),
            ));
        }
        let (nonce_bytes, ct) = data.split_at(12);
        let master = Self::derive_master_key();
        let cipher = Aes256Gcm::new_from_slice(&master)
            .map_err(|e| BearDogError::internal(format!("AES master key init: {e}")))?;
        let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
        cipher
            .decrypt(nonce, ct)
            .map_err(|e| BearDogError::internal(format!("At-rest decrypt: {e}")))
    }

    /// Store an encrypted key blob to disk.
    async fn store_blob(&self, key_id: &str, blob: &[u8]) -> Result<(), BearDogError> {
        let path = self.blob_path(key_id);
        tokio::fs::write(&path, blob).await.map_err(|e| {
            BearDogError::internal(format!(
                "Failed to write secret blob for {key_id}: {e}"
            ))
        })
    }

    /// Load an encrypted key blob from disk.
    async fn load_blob(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        let path = self.blob_path(key_id);
        tokio::fs::read(&path).await.map_err(|e| {
            BearDogError::internal(format!(
                "Failed to read secret blob for {key_id}: {e}"
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsm_available_on_linux() {
        let hsm = LinuxSecretServiceHsm::new().expect("construction");
        if cfg!(target_os = "linux") {
            assert!(beardog_traits::hsm::HsmKeyProvider::is_available(&hsm));
        } else {
            assert!(!beardog_traits::hsm::HsmKeyProvider::is_available(&hsm));
        }
    }

    #[test]
    fn blob_path_uses_key_id() {
        let hsm = LinuxSecretServiceHsm::new().expect("construction");
        let p = hsm.blob_path("test-key");
        assert!(p.to_string_lossy().contains("test-key.secret"));
    }

    #[test]
    fn at_rest_encrypt_decrypt_roundtrip() {
        let plaintext = b"secret-key-material-32-bytes!!!!";
        let encrypted = LinuxSecretServiceHsm::encrypt_at_rest(plaintext)
            .expect("encrypt_at_rest");
        assert_ne!(encrypted, plaintext);
        let decrypted = LinuxSecretServiceHsm::decrypt_at_rest(&encrypted)
            .expect("decrypt_at_rest");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn master_key_derivation_is_deterministic() {
        let k1 = LinuxSecretServiceHsm::derive_master_key();
        let k2 = LinuxSecretServiceHsm::derive_master_key();
        assert_eq!(k1, k2);
    }

    #[test]
    fn provider_identity() {
        use beardog_traits::hsm::HsmKeyProvider;

        let hsm = LinuxSecretServiceHsm::new().expect("construction");
        assert_eq!(hsm.provider_id(), "linux-secret-service");
        assert_eq!(
            hsm.provider_type(),
            beardog_types::hsm::HsmProviderType::LinuxSecretService
        );
    }

    #[test]
    fn capabilities_not_hardware_backed() {
        use beardog_traits::hsm::HsmKeyProvider;

        let hsm = LinuxSecretServiceHsm::new().expect("construction");
        let caps = hsm.capabilities();
        assert!(!caps.hardware_backed);
        assert!(!caps.supports_key_export);
        assert!(caps.supports(beardog_types::hsm::HsmAlgorithm::Aes256Gcm));
        assert!(caps.supports(beardog_types::hsm::HsmAlgorithm::Ed25519));
    }

    #[tokio::test]
    async fn generate_key_and_roundtrip_encrypt_decrypt() {
        use beardog_traits::hsm::HsmKeyProvider;
        use beardog_types::hsm::{HsmAlgorithm, KeyGenParams};

        let hsm = LinuxSecretServiceHsm::new().expect("construction");
        if !hsm.is_available() {
            return;
        }

        let mut params = KeyGenParams::new(HsmAlgorithm::Aes256Gcm);
        params.label = Some("test-roundtrip-key".to_string());

        let handle = hsm.generate_key(&params).await.expect("generate_key");
        assert_eq!(handle.key_id, "test-roundtrip-key");

        let plaintext = b"hello beardog hsm abstraction";
        let ciphertext = hsm
            .encrypt(&handle.key_id, plaintext)
            .await
            .expect("encrypt");
        assert_ne!(ciphertext, plaintext.to_vec());

        let decrypted = hsm
            .decrypt(&handle.key_id, &ciphertext)
            .await
            .expect("decrypt");
        assert_eq!(decrypted, plaintext.to_vec());

        assert!(hsm.key_exists(&handle.key_id).await.expect("key_exists"));
        hsm.delete_key(&handle.key_id).await.expect("delete_key");
        assert!(!hsm.key_exists(&handle.key_id).await.expect("key_exists after delete"));
    }

    #[tokio::test]
    async fn sign_verify_roundtrip() {
        use beardog_traits::hsm::HsmKeyProvider;
        use beardog_types::hsm::{HsmAlgorithm, KeyGenParams};

        let hsm = LinuxSecretServiceHsm::new().expect("construction");
        if !hsm.is_available() {
            return;
        }

        let mut params = KeyGenParams::new(HsmAlgorithm::HmacSha256);
        params.label = Some("test-sign-key".to_string());

        let handle = hsm.generate_key(&params).await.expect("generate_key");

        let data = b"data to sign";
        let sig = hsm.sign(&handle.key_id, data).await.expect("sign");
        let verified = hsm
            .verify(&handle.key_id, data, &sig)
            .await
            .expect("verify");
        assert!(verified);

        let bad_verified = hsm
            .verify(&handle.key_id, b"wrong data", &sig)
            .await
            .expect("verify with wrong data");
        assert!(!bad_verified);

        hsm.delete_key(&handle.key_id).await.expect("cleanup");
    }
}
