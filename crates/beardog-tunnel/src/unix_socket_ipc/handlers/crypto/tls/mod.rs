// SPDX-License-Identifier: AGPL-3.0-or-later

//! TLS 1.3 cryptographic operations for HTTPS
//!
//! This module provides TLS 1.3 key derivation, signatures, and certificate
//! verification methods according to RFC 8446. All operations follow the
//! official TLS 1.3 specifications and use Pure Rust implementations.
//!
//! # Architecture
//!
//! The TLS module is organized into focused sub-modules:
//!
//! - [`key_derivation`]: HKDF-based key schedule (handshake & application secrets)
//! - [`signatures`]: Ed25519 signing and Finished MAC computation
//! - [`certificates`]: X.509 certificate chain verification
//!
//! # TLS 1.3 Key Schedule (RFC 8446 Section 7.1)
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
//! # Handler Methods
//!
//! ## Key Derivation (RFC 8446 Section 7.1)
//!
//! - [`handle_tls_derive_secrets`] - Legacy combined key derivation
//! - [`handle_tls_derive_handshake_secrets`] - Handshake traffic keys (Stage 1)
//! - [`handle_tls_derive_application_secrets`] - Application traffic keys (Stage 2)
//!
//! ## Signatures (RFC 8032, RFC 8446 Section 4.4)
//!
//! - [`handle_tls_sign_handshake`] - Sign handshake messages with Ed25519
//! - [`handle_tls_compute_finished_verify_data`] - Compute TLS Finished MAC
//!
//! ## Certificate Verification (RFC 5280, RFC 6125)
//!
//! - [`handle_tls_verify_certificate`] - Parse and validate X.509 certificates
//!
//! # Usage Example
//!
//! ```rust,ignore
//! // NOTE: These handlers are internal and called via JSON-RPC
//! use crate::unix_socket_ipc::handlers::crypto::tls::*;
//!
//! // 1. Derive handshake keys (Stage 1)
//! let handshake_keys = handle_tls_derive_handshake_secrets(params).await?;
//!
//! // 2. Derive application keys (Stage 2)
//! let application_keys = handle_tls_derive_application_secrets(params).await?;
//!
//! // 3. Sign handshake transcript
//! let signature = handle_tls_sign_handshake(params).await?;
//!
//! // 4. Compute Finished message
//! let finished_mac = handle_tls_compute_finished_verify_data(params).await?;
//!
//! // 5. Verify certificate chain
//! let cert_info = handle_tls_verify_certificate(params).await?;
//! ```
//!
//! # Standards Compliance
//!
//! - ✅ RFC 8446: TLS 1.3
//! - ✅ RFC 5869: HKDF
//! - ✅ RFC 8032: Ed25519
//! - ✅ RFC 5280: X.509
//! - ✅ RFC 6125: Server Identity
//! - ✅ RFC 2104: HMAC
//!
//! # Security Properties
//!
//! - **Pure Rust**: No unchecked memory patterns, no C dependencies
//! - **Constant-time**: Crypto operations resist timing attacks
//! - **Forward secrecy**: Ephemeral keys via X25519
//! - **Key confirmation**: Finished message binds transcript
//! - **Zero-copy**: Optimized for minimal allocations
//!
//! # References
//!
//! - RFC 8446 (TLS 1.3): <https://www.rfc-editor.org/rfc/rfc8446.html>
//! - RFC 5869 (HKDF): <https://www.rfc-editor.org/rfc/rfc5869.html>
//! - RFC 8032 (Ed25519): <https://www.rfc-editor.org/rfc/rfc8032.html>
//! - RFC 5280 (X.509): <https://www.rfc-editor.org/rfc/rfc5280.html>
//! - RFC 6125 (Identity): <https://www.rfc-editor.org/rfc/rfc6125.html>

// Sub-modules
#[cfg(feature = "tls-x509")]
pub mod certificates;
pub mod key_derivation;
pub mod signatures;

// Re-export all public handlers for backward compatibility
#[cfg(feature = "tls-x509")]
pub use certificates::handle_tls_verify_certificate;
pub use key_derivation::{
    handle_tls_derive_application_secrets, handle_tls_derive_handshake_secrets,
    handle_tls_derive_secrets,
};
pub use signatures::{handle_tls_compute_finished_verify_data, handle_tls_sign_handshake};
