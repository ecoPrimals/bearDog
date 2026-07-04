// SPDX-License-Identifier: AGPL-3.0-or-later

//! Process-wide rustls `CryptoProvider` installation (RustCrypto backend).

/// Install the pure-Rust RustCrypto-based rustls crypto provider once per process.
pub(crate) fn ensure_installed() {
    static INSTALL: std::sync::Once = std::sync::Once::new();
    INSTALL.call_once(|| {
        let _ = rustls_rustcrypto::provider().install_default();
    });
}
