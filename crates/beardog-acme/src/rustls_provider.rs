// SPDX-License-Identifier: AGPL-3.0-or-later

//! Process-wide rustls `CryptoProvider` management.
//!
//! The binary (`beardog-cli`) installs the provider via `rustls-rustcrypto`
//! at startup. This module provides a self-healing fallback: if no provider
//! is found, it attempts installation automatically.

/// Ensure that a rustls `CryptoProvider` is available.
///
/// Installs `rustls-rustcrypto` as the process-wide default if no provider
/// has been set yet. Idempotent — safe to call from multiple sites.
pub fn assert_installed() {
    if rustls::crypto::CryptoProvider::get_default().is_some() {
        return;
    }

    let _ = rustls_rustcrypto::provider().install_default();
}
