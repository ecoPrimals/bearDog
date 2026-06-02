// SPDX-License-Identifier: AGPL-3.0-or-later

//! Certificate storage — filesystem persistence for PEM certificates and keys.

use beardog_config::env_keys;

use crate::error::AcmeError;
use std::path::{Path, PathBuf};
use tracing::info;

/// Manages on-disk certificate storage at `$BEARDOG_DATA_DIR/acme/`.
#[derive(Debug, Clone)]
pub struct CertificateStore {
    /// Root directory for all ACME data.
    base_dir: PathBuf,
}

impl CertificateStore {
    /// Create a new certificate store rooted at the given directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be created.
    pub fn new(base_dir: &Path) -> Result<Self, AcmeError> {
        std::fs::create_dir_all(base_dir)?;
        Ok(Self {
            base_dir: base_dir.to_path_buf(),
        })
    }

    /// Resolve from environment: `$BEARDOG_DATA_DIR/acme/` or `~/.beardog/acme/`.
    ///
    /// # Errors
    ///
    /// Returns an error if the resolved directory cannot be created.
    pub fn from_env() -> Result<Self, AcmeError> {
        let base = if let Ok(data_dir) = std::env::var(env_keys::ENV_DATA_DIR) {
            PathBuf::from(data_dir).join("acme")
        } else if let Ok(home) = std::env::var(env_keys::ENV_HOME) {
            PathBuf::from(home).join(".beardog").join("acme")
        } else {
            PathBuf::from("/tmp/beardog-acme")
        };
        Self::new(&base)
    }

    /// Path to the account file.
    #[must_use]
    pub fn account_path(&self) -> PathBuf {
        self.base_dir.join("account.json")
    }

    /// Path to the certificate directory for a given domain.
    #[must_use]
    pub fn cert_dir(&self, domain: &str) -> PathBuf {
        self.base_dir.join("certs").join(domain)
    }

    /// Path to the fullchain PEM for a domain.
    #[must_use]
    pub fn fullchain_path(&self, domain: &str) -> PathBuf {
        self.cert_dir(domain).join("fullchain.pem")
    }

    /// Path to the private key PEM for a domain.
    #[must_use]
    pub fn privkey_path(&self, domain: &str) -> PathBuf {
        self.cert_dir(domain).join("privkey.pem")
    }

    /// Store a certificate and key for a domain.
    ///
    /// # Errors
    ///
    /// Returns an error if directories cannot be created or files written.
    pub fn store_cert(
        &self,
        domain: &str,
        fullchain_pem: &str,
        privkey_pem: &str,
    ) -> Result<(), AcmeError> {
        let cert_dir = self.cert_dir(domain);
        std::fs::create_dir_all(&cert_dir)?;

        std::fs::write(self.fullchain_path(domain), fullchain_pem)?;

        let key_path = self.privkey_path(domain);
        std::fs::write(&key_path, privkey_pem)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600))?;
        }

        info!(domain, "stored certificate and key");
        Ok(())
    }

    /// Load a certificate from storage. Returns `None` if not found.
    ///
    /// # Errors
    ///
    /// Returns an error if files exist but cannot be read.
    pub fn load_cert(&self, domain: &str) -> Result<Option<StoredCert>, AcmeError> {
        let chain_path = self.fullchain_path(domain);
        let key_path = self.privkey_path(domain);

        if !chain_path.exists() || !key_path.exists() {
            return Ok(None);
        }

        let fullchain_pem = std::fs::read_to_string(&chain_path)?;
        let privkey_pem = std::fs::read_to_string(&key_path)?;

        Ok(Some(StoredCert {
            fullchain_pem,
            privkey_pem,
        }))
    }
}

/// A certificate and its private key loaded from storage.
#[derive(Debug, Clone)]
pub struct StoredCert {
    /// PEM-encoded certificate chain.
    pub fullchain_pem: String,
    /// PEM-encoded private key.
    pub privkey_pem: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn store_and_load_roundtrip() {
        let dir = tempdir().expect("tempdir");
        let store = CertificateStore::new(dir.path()).expect("store");

        let chain = "-----BEGIN CERTIFICATE-----\nfake\n-----END CERTIFICATE-----\n";
        let key = "-----BEGIN PRIVATE KEY-----\nsecret\n-----END PRIVATE KEY-----\n";

        store.store_cert("example.com", chain, key).expect("store");
        let loaded = store
            .load_cert("example.com")
            .expect("load")
            .expect("exists");

        assert_eq!(loaded.fullchain_pem, chain);
        assert_eq!(loaded.privkey_pem, key);
    }

    #[test]
    fn load_returns_none_when_missing() {
        let dir = tempdir().expect("tempdir");
        let store = CertificateStore::new(dir.path()).expect("store");
        assert!(store.load_cert("missing.com").expect("load").is_none());
    }

    #[test]
    fn paths_are_domain_scoped() {
        let dir = tempdir().expect("tempdir");
        let store = CertificateStore::new(dir.path()).expect("store");
        let chain_path = store.fullchain_path("foo.bar");
        assert!(chain_path.to_string_lossy().contains("foo.bar"));
        assert!(chain_path.to_string_lossy().contains("fullchain.pem"));
    }

    #[test]
    fn account_path_is_at_root() {
        let dir = tempdir().expect("tempdir");
        let store = CertificateStore::new(dir.path()).expect("store");
        let path = store.account_path();
        assert!(path.ends_with("account.json"));
    }
}
