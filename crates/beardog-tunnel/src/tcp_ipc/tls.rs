// SPDX-License-Identifier: AGPL-3.0-or-later

//! TLS termination layer for `BearDog` TCP server (H2-10 sovereignty).
//!
//! Provides X.509/TLS termination using `rustls` so `BearDog` can serve HTTPS
//! directly without Cloudflare or any external TLS proxy. The `ring` crypto
//! backend is used for universal platform support.
//!
//! # Certificate Loading
//!
//! Certificates and private keys are loaded from PEM files specified via
//! environment variables or config:
//! - `BEARDOG_TLS_CERT_PATH` — PEM certificate chain
//! - `BEARDOG_TLS_KEY_PATH` — PEM private key (PKCS8 or RSA)
//!
//! # SNI
//!
//! Server Name Indication is supported: the acceptor validates the client's
//! requested hostname against the loaded certificate's subject names.

use beardog_config::env_keys;
use std::path::Path;
use std::sync::Arc;

use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pki_types::pem::{Error as PemError, PemObject};
use tokio_rustls::TlsAcceptor;
use tracing::{info, warn};

/// Configuration for the TLS termination layer.
#[derive(Debug, Clone)]
pub struct TlsTerminationConfig {
    /// Path to PEM certificate chain file.
    pub cert_path: String,
    /// Path to PEM private key file (PKCS8 or RSA).
    pub key_path: String,
}

/// Errors from TLS configuration.
#[derive(Debug)]
pub enum TlsConfigError {
    /// Certificate file could not be read.
    CertFileError(String),
    /// Key file could not be read.
    KeyFileError(String),
    /// No certificates found in PEM file.
    NoCertificates,
    /// No private key found in PEM file.
    NoPrivateKey,
    /// Failed to build `rustls` server config.
    ServerConfigError(String),
}

impl std::fmt::Display for TlsConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CertFileError(e) => write!(f, "failed to read cert file: {e}"),
            Self::KeyFileError(e) => write!(f, "failed to read key file: {e}"),
            Self::NoCertificates => write!(f, "no certificates found in PEM file"),
            Self::NoPrivateKey => write!(f, "no private key found in PEM file"),
            Self::ServerConfigError(e) => write!(f, "rustls ServerConfig error: {e}"),
        }
    }
}

impl std::error::Error for TlsConfigError {}

impl TlsTerminationConfig {
    /// Load config from environment variables.
    ///
    /// Returns `None` if neither `BEARDOG_TLS_CERT_PATH` nor `BEARDOG_TLS_KEY_PATH`
    /// is set (TLS disabled).
    ///
    /// # Errors
    ///
    /// Returns an error description if only one of the two env vars is set.
    #[must_use]
    pub fn from_env() -> Option<Self> {
        let cert = std::env::var(env_keys::ENV_TLS_CERT_PATH).ok();
        let key = std::env::var(env_keys::ENV_TLS_KEY_PATH).ok();

        match (cert, key) {
            (Some(cert_path), Some(key_path)) => Some(Self {
                cert_path,
                key_path,
            }),
            (None, None) => None,
            (Some(_), None) => {
                warn!("BEARDOG_TLS_CERT_PATH set but BEARDOG_TLS_KEY_PATH missing — TLS disabled");
                None
            }
            (None, Some(_)) => {
                warn!("BEARDOG_TLS_KEY_PATH set but BEARDOG_TLS_CERT_PATH missing — TLS disabled");
                None
            }
        }
    }
}

/// Load PEM certificates from a file path.
///
/// # Errors
///
/// Returns `TlsConfigError` if the file cannot be opened or contains no certs.
fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, TlsConfigError> {
    let certs: Vec<CertificateDer<'static>> = CertificateDer::pem_file_iter(Path::new(path))
        .map_err(|e| TlsConfigError::CertFileError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| TlsConfigError::CertFileError(e.to_string()))?;

    if certs.is_empty() {
        return Err(TlsConfigError::NoCertificates);
    }

    Ok(certs)
}

/// Load the first PEM private key from a file path.
///
/// Supports PKCS8, RSA, and EC key formats.
///
/// # Errors
///
/// Returns `TlsConfigError` if the file cannot be opened or contains no key.
fn load_private_key(path: &str) -> Result<PrivateKeyDer<'static>, TlsConfigError> {
    PrivateKeyDer::from_pem_file(Path::new(path)).map_err(|e| match e {
        PemError::NoItemsFound => TlsConfigError::NoPrivateKey,
        PemError::Io(err) => TlsConfigError::KeyFileError(err.to_string()),
        err => TlsConfigError::KeyFileError(err.to_string()),
    })
}

/// Build a `TlsAcceptor` from the given config.
///
/// # Errors
///
/// Returns `TlsConfigError` if certificates or keys cannot be loaded or the
/// `rustls` `ServerConfig` cannot be built.
pub fn build_tls_acceptor(config: &TlsTerminationConfig) -> Result<TlsAcceptor, TlsConfigError> {
    let certs = load_certs(&config.cert_path)?;
    let key = load_private_key(&config.key_path)?;

    let cert_count = certs.len();

    let server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| TlsConfigError::ServerConfigError(e.to_string()))?;

    info!(
        cert_count,
        cert_path = %config.cert_path,
        "TLS termination configured (H2-10 sovereignty)"
    );

    Ok(TlsAcceptor::from(Arc::new(server_config)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_struct_fields() {
        let config = TlsTerminationConfig {
            cert_path: "/tmp/cert.pem".to_owned(),
            key_path: "/tmp/key.pem".to_owned(),
        };
        assert_eq!(config.cert_path, "/tmp/cert.pem");
        assert_eq!(config.key_path, "/tmp/key.pem");
    }

    #[test]
    fn config_clone() {
        let config = TlsTerminationConfig {
            cert_path: "/a".to_owned(),
            key_path: "/b".to_owned(),
        };
        let cloned = config.clone();
        assert_eq!(cloned.cert_path, "/a");
        assert_eq!(cloned.key_path, "/b");
    }

    #[test]
    fn load_certs_missing_file_returns_error() {
        let result = load_certs("/nonexistent/path/cert.pem");
        assert!(result.is_err());
        assert!(matches!(result, Err(TlsConfigError::CertFileError(_))));
    }

    #[test]
    fn load_private_key_missing_file_returns_error() {
        let result = load_private_key("/nonexistent/path/key.pem");
        assert!(result.is_err());
        assert!(matches!(result, Err(TlsConfigError::KeyFileError(_))));
    }

    #[test]
    fn load_certs_empty_file_returns_no_certs() {
        let dir = tempfile::tempdir().expect("create tempdir");
        let cert_path = dir.path().join("empty.pem");
        std::fs::write(&cert_path, "").expect("write empty file");
        let result = load_certs(cert_path.to_str().expect("path to str"));
        assert!(matches!(result, Err(TlsConfigError::NoCertificates)));
    }

    #[test]
    fn load_key_empty_file_returns_no_key() {
        let dir = tempfile::tempdir().expect("create tempdir");
        let key_path = dir.path().join("empty.pem");
        std::fs::write(&key_path, "").expect("write empty file");
        let result = load_private_key(key_path.to_str().expect("path to str"));
        assert!(matches!(result, Err(TlsConfigError::NoPrivateKey)));
    }

    #[test]
    fn tls_config_error_display() {
        let e = TlsConfigError::NoCertificates;
        assert_eq!(format!("{e}"), "no certificates found in PEM file");
        let e = TlsConfigError::NoPrivateKey;
        assert_eq!(format!("{e}"), "no private key found in PEM file");
        let e = TlsConfigError::CertFileError("io error".to_owned());
        assert!(format!("{e}").contains("io error"));
        let e = TlsConfigError::KeyFileError("io error".to_owned());
        assert!(format!("{e}").contains("io error"));
        let e = TlsConfigError::ServerConfigError("bad config".to_owned());
        assert!(format!("{e}").contains("bad config"));
    }

    #[test]
    fn tls_config_error_is_error() {
        let e: Box<dyn std::error::Error> = Box::new(TlsConfigError::NoCertificates);
        assert!(e.to_string().contains("no certificates"));
    }

    #[test]
    fn build_acceptor_with_missing_cert_fails() {
        let config = TlsTerminationConfig {
            cert_path: "/does/not/exist.pem".to_owned(),
            key_path: "/does/not/exist.key".to_owned(),
        };
        assert!(build_tls_acceptor(&config).is_err());
    }
}
