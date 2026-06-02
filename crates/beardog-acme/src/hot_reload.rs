// SPDX-License-Identifier: AGPL-3.0-or-later

//! Hot-reload mechanism for TLS certificates.
//!
//! Provides an atomic swap layer over `rustls::ServerConfig` so that new
//! certificates (obtained via ACME) can be loaded without restarting the
//! server. Active connections continue on the old certificate; new connections
//! use the refreshed certificate.

use crate::error::AcmeError;
use crate::storage::CertificateStore;
use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pki_types::pem::PemObject;
use std::sync::Arc;
use tokio::sync::watch;
use tokio_rustls::TlsAcceptor;
use tracing::info;

/// A TLS acceptor that can be hot-reloaded when certificates change.
///
/// Uses a `watch` channel internally so all active listeners receive the
/// updated `TlsAcceptor` without coordination.
#[derive(Clone)]
pub struct HotReloadAcceptor {
    rx: watch::Receiver<Arc<TlsAcceptor>>,
}

impl HotReloadAcceptor {
    /// Get the current `TlsAcceptor`.
    #[must_use]
    pub fn current(&self) -> Arc<TlsAcceptor> {
        Arc::clone(&*self.rx.borrow())
    }

    /// Wait until the acceptor changes and return the new one.
    pub async fn changed(&mut self) -> Arc<TlsAcceptor> {
        let _ = self.rx.changed().await;
        self.current()
    }
}

/// Controller for hot-reloading TLS certificates.
///
/// Holds the send side of the watch channel. When a new certificate is
/// available, call `reload` to atomically update all listeners.
pub struct HotReloadController {
    tx: watch::Sender<Arc<TlsAcceptor>>,
}

impl HotReloadController {
    /// Reload the TLS acceptor with fresh certificates from the store.
    ///
    /// # Errors
    ///
    /// Returns an error if the certificate cannot be loaded or parsed.
    pub fn reload_from_store(
        &self,
        store: &CertificateStore,
        domain: &str,
    ) -> Result<(), AcmeError> {
        let cert = store
            .load_cert(domain)?
            .ok_or_else(|| AcmeError::CertParse(format!("no cert for {domain}")))?;

        let acceptor = build_acceptor_from_pem(&cert.fullchain_pem, &cert.privkey_pem)?;
        self.tx
            .send(Arc::new(acceptor))
            .map_err(|_| AcmeError::Config("all receivers dropped".to_string()))?;

        info!(domain, "hot-reloaded TLS certificate");
        Ok(())
    }

    /// Reload with explicit PEM data (for testing or external cert sources).
    ///
    /// # Errors
    ///
    /// Returns an error if the PEM data cannot be parsed.
    pub fn reload_from_pem(&self, fullchain_pem: &str, privkey_pem: &str) -> Result<(), AcmeError> {
        let acceptor = build_acceptor_from_pem(fullchain_pem, privkey_pem)?;
        self.tx
            .send(Arc::new(acceptor))
            .map_err(|_| AcmeError::Config("all receivers dropped".to_string()))?;
        Ok(())
    }
}

/// Create a hot-reload TLS pair from initial PEM certificate and key.
///
/// Returns both the acceptor (for use by the TLS listener) and the controller
/// (for triggering reloads when ACME obtains a new cert).
///
/// # Errors
///
/// Returns an error if the initial certificate cannot be parsed.
pub fn create_hot_reload_pair(
    fullchain_pem: &str,
    privkey_pem: &str,
) -> Result<(HotReloadAcceptor, HotReloadController), AcmeError> {
    let acceptor = build_acceptor_from_pem(fullchain_pem, privkey_pem)?;
    let (tx, rx) = watch::channel(Arc::new(acceptor));

    Ok((HotReloadAcceptor { rx }, HotReloadController { tx }))
}

/// Build a `TlsAcceptor` from in-memory PEM data.
fn build_acceptor_from_pem(
    fullchain_pem: &str,
    privkey_pem: &str,
) -> Result<TlsAcceptor, AcmeError> {
    let certs = load_certs_from_pem(fullchain_pem)?;
    let key = load_key_from_pem(privkey_pem)?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| AcmeError::CertParse(format!("ServerConfig: {e}")))?;

    Ok(TlsAcceptor::from(Arc::new(config)))
}

/// Parse PEM certificate chain from a string.
fn load_certs_from_pem(pem: &str) -> Result<Vec<CertificateDer<'static>>, AcmeError> {
    let certs: Vec<CertificateDer<'static>> = CertificateDer::pem_slice_iter(pem.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AcmeError::CertParse(format!("certificate PEM: {e}")))?;

    if certs.is_empty() {
        return Err(AcmeError::CertParse("no certificates in PEM".to_string()));
    }
    Ok(certs)
}

/// Parse a private key from PEM string (PKCS8, PKCS1, or SEC1).
fn load_key_from_pem(pem: &str) -> Result<PrivateKeyDer<'static>, AcmeError> {
    PrivateKeyDer::from_pem_slice(pem.as_bytes())
        .map_err(|e| AcmeError::CertParse(format!("private key PEM: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_certs_from_pem_rejects_empty() {
        let result = load_certs_from_pem("");
        assert!(result.is_err());
    }

    #[test]
    fn load_key_from_pem_rejects_empty() {
        let result = load_key_from_pem("");
        assert!(result.is_err());
    }

    #[test]
    fn load_certs_from_pem_rejects_garbage() {
        let result = load_certs_from_pem("not a pem file at all");
        assert!(result.is_err());
    }
}
