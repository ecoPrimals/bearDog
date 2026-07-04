// SPDX-License-Identifier: AGPL-3.0-or-later

//! Process-wide rustls `CryptoProvider` management.
//!
//! In production, the binary (`beardog-cli`) installs the provider via
//! `rustls-rustcrypto` at startup. This module asserts it's available.
//! In test mode, it installs the provider directly (via dev-dependency).

/// Ensure that a rustls `CryptoProvider` is available.
///
/// In production: asserts the provider was installed by the binary.
/// In tests: installs the provider if not already present.
///
/// # Panics
///
/// Panics in production if no provider has been installed.
pub fn assert_installed() {
    if rustls::crypto::CryptoProvider::get_default().is_some() {
        return;
    }

    #[cfg(test)]
    {
        let _ = rustls_rustcrypto::provider().install_default();
        return;
    }

    #[cfg(not(test))]
    panic!(
        "rustls CryptoProvider not installed — binary must call \
         rustls_rustcrypto::provider().install_default() at startup"
    );
}
