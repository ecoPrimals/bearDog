// SPDX-License-Identifier: AGPL-3.0-only

//! TLS 1.3 Key Derivation Functions
//!
//! This module implements the complete TLS 1.3 key schedule according to RFC 8446 Section 7.1.
//! It provides both handshake and application traffic secret derivation using HKDF-SHA256.
//!
//! # TLS 1.3 Key Schedule
//!
//! ```text
//!              0
//!              |
//!              v
//!    PSK ->  HKDF-Extract = Early Secret
//!              |
//!              v
//!        Derive-Secret(., "derived", "")
//!              |
//!              v
//! (EC)DHE -> HKDF-Extract = Handshake Secret
//!              |
//!              +-----> Derive-Secret(., "c hs traffic", transcript)
//!              |       = client_handshake_traffic_secret
//!              |
//!              +-----> Derive-Secret(., "s hs traffic", transcript)
//!              |       = server_handshake_traffic_secret
//!              v
//!        Derive-Secret(., "derived", "")
//!              |
//!              v
//!        0 -> HKDF-Extract = Master Secret
//!              |
//!              +-----> Derive-Secret(., "c ap traffic", transcript)
//!              |       = client_application_traffic_secret_0
//!              |
//!              +-----> Derive-Secret(., "s ap traffic", transcript)
//!                      = server_application_traffic_secret_0
//! ```
//!
//! # Exports
//!
//! - [`handle_tls_derive_secrets`] - Legacy combined derivation
//! - [`handle_tls_derive_handshake_secrets`] - Handshake traffic keys
//! - [`handle_tls_derive_application_secrets`] - Application traffic keys
//!
//! # References
//!
//! - RFC 8446: TLS 1.3
//! - RFC 5869: HKDF

// Internal helper functions for TLS 1.3 key derivation (SHA-256 and SHA-384)
#[path = "key_derivation_helpers.rs"]
mod helpers;
pub(crate) use helpers::append_tls13_hkdf_label;

#[path = "derive_application.rs"]
mod derive_application;
#[path = "derive_handshake.rs"]
mod derive_handshake;
#[path = "derive_secrets.rs"]
mod derive_secrets;

pub use derive_application::handle_tls_derive_application_secrets;
pub use derive_handshake::handle_tls_derive_handshake_secrets;
pub use derive_secrets::handle_tls_derive_secrets;

#[cfg(test)]
#[path = "key_derivation_tests.rs"]
mod tests;
