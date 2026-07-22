// SPDX-License-Identifier: AGPL-3.0-or-later

//! Silicon Atheism enum dispatch for [`CredentialStore`] backends.
//!
//! All backends compile on all platforms. Platform-specific backends
//! (DPAPI, Keychain, Keystore) can be added as variants without `#[cfg]`
//! gating — they return appropriate errors when unavailable.

use super::android_keystore::AndroidKeystoreCredentialStore;
use super::file_vault::FileVaultCredentialStore;
use super::in_memory::InMemoryCredentialStore;
use beardog_errors::BearDogError;
use beardog_traits::unified::storage::{CredentialStore, SecretMetadata};

/// Runtime-dispatched credential store backend.
///
/// Follows the Silicon Atheism pattern: enum variants rather than trait
/// objects, giving monomorphic dispatch while supporting runtime backend
/// selection.
#[derive(Debug)]
pub enum CredentialStoreBackend {
    /// Volatile in-memory store (dev/test, bootstrap cache).
    InMemory(InMemoryCredentialStore),
    /// Persistent encrypted file vault (production default).
    FileVault(FileVaultCredentialStore),
    /// Android Keystore hardware-backed store (grapheneGate / mobile).
    AndroidKeystore(AndroidKeystoreCredentialStore),
}

impl CredentialStoreBackend {
    /// Create an in-memory backend.
    #[must_use]
    pub fn in_memory() -> Self {
        Self::InMemory(InMemoryCredentialStore::new())
    }

    /// Create a file vault backend.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if the vault directory cannot be created.
    pub fn file_vault(
        vault_dir: std::path::PathBuf,
        master_key: [u8; 32],
    ) -> Result<Self, BearDogError> {
        Ok(Self::FileVault(FileVaultCredentialStore::new(
            vault_dir, master_key,
        )?))
    }

    /// Create an Android Keystore backend.
    ///
    /// The master key is retrieved from (or generated in) the Android Keystore
    /// hardware enclave. Only available on `target_os = "android"`.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] if Android Keystore is not available or the
    /// vault directory cannot be created.
    pub fn android_keystore(
        vault_dir: std::path::PathBuf,
    ) -> Result<Self, BearDogError> {
        Ok(Self::AndroidKeystore(
            AndroidKeystoreCredentialStore::new(vault_dir)?,
        ))
    }
}

impl CredentialStore for CredentialStoreBackend {
    async fn store(&self, name: &str, value: &str) -> Result<(), BearDogError> {
        match self {
            Self::InMemory(s) => s.store(name, value).await,
            Self::FileVault(s) => s.store(name, value).await,
            Self::AndroidKeystore(s) => s.store(name, value).await,
        }
    }

    async fn retrieve(&self, name: &str) -> Result<(String, SecretMetadata), BearDogError> {
        match self {
            Self::InMemory(s) => s.retrieve(name).await,
            Self::FileVault(s) => s.retrieve(name).await,
            Self::AndroidKeystore(s) => s.retrieve(name).await,
        }
    }

    async fn list(&self) -> Result<Vec<String>, BearDogError> {
        match self {
            Self::InMemory(s) => s.list().await,
            Self::FileVault(s) => s.list().await,
            Self::AndroidKeystore(s) => s.list().await,
        }
    }

    async fn delete(&self, name: &str) -> Result<bool, BearDogError> {
        match self {
            Self::InMemory(s) => s.delete(name).await,
            Self::FileVault(s) => s.delete(name).await,
            Self::AndroidKeystore(s) => s.delete(name).await,
        }
    }

    fn backend_id(&self) -> &'static str {
        match self {
            Self::InMemory(s) => s.backend_id(),
            Self::FileVault(s) => s.backend_id(),
            Self::AndroidKeystore(s) => s.backend_id(),
        }
    }

    fn is_persistent(&self) -> bool {
        match self {
            Self::InMemory(s) => s.is_persistent(),
            Self::FileVault(s) => s.is_persistent(),
            Self::AndroidKeystore(s) => s.is_persistent(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn dispatch_in_memory() {
        let backend = CredentialStoreBackend::in_memory();
        assert_eq!(backend.backend_id(), "in-memory");
        assert!(!backend.is_persistent());

        backend.store("k", "v").await.unwrap();
        let (val, _) = backend.retrieve("k").await.unwrap();
        assert_eq!(val, "v");
    }

    #[tokio::test]
    async fn dispatch_file_vault() {
        let dir = tempfile::tempdir().unwrap();
        let backend =
            CredentialStoreBackend::file_vault(dir.path().join("vault"), [99u8; 32]).unwrap();
        assert_eq!(backend.backend_id(), "file-vault");
        assert!(backend.is_persistent());

        backend.store("key", "secret").await.unwrap();
        let (val, _) = backend.retrieve("key").await.unwrap();
        assert_eq!(val, "secret");

        let names = backend.list().await.unwrap();
        assert_eq!(names, vec!["key"]);

        assert!(backend.delete("key").await.unwrap());
        assert!(backend.retrieve("key").await.is_err());
    }

    #[test]
    fn android_keystore_unavailable_on_non_android() {
        let dir = tempfile::tempdir().unwrap();
        let result = CredentialStoreBackend::android_keystore(dir.path().join("vault"));
        assert!(result.is_err());
    }
}
