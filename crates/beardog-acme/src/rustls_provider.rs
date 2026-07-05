// SPDX-License-Identifier: AGPL-3.0-or-later

//! Process-wide rustls `CryptoProvider` management.
//!
//! In production, the binary (`beardog-cli`) installs the provider via
//! `rustls-rustcrypto` at startup. This module asserts it's available.
//! In test mode, it installs the provider directly (via dev-dependency).

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
