// SPDX-License-Identifier: AGPL-3.0-or-later

//! # beardog-crypto
//!
//! Pure Rust cryptographic primitives for the `BearDog` Tower Atomic pattern.
//!
//! This crate extracts the core cryptographic algorithms from `beardog-core`
//! into a standalone, lean crate suitable for:
//!
//! - Direct Rust linking (no IPC overhead)
//! - `cdylib` extraction for `libtower.so` (Chimera Phase 0)
//! - Independent publishing to crates.io
//!
//! ## Algorithm Coverage
//!
//! | Category | Algorithms |
//! |----------|-----------|
//! | **Signatures** | Ed25519, ECDSA (P-256), RSA-PSS |
//! | **Key Exchange** | X25519 |
//! | **AEAD** | ChaCha20-Poly1305, AES-128/256-GCM |
//! | **Hashing** | BLAKE3, SHA-256/384/512, SHA3-256 |
//! | **MAC** | HMAC-SHA256/384/512 |
//! | **KDF** | HKDF-SHA256 |
//! | **Passwords** | Argon2id |
//!
//! All implementations are 100% pure Rust with zero C dependencies.

#![forbid(unsafe_code)]

/// Symmetric encryption (AEAD): AES-GCM, ChaCha20-Poly1305.
pub mod symmetric;

/// Asymmetric cryptography: Ed25519, X25519, ECDSA P-256, RSA.
pub mod asymmetric;

/// Hashing: BLAKE3, SHA-2, SHA-3, HMAC, HKDF, Argon2id.
pub mod hashing;

/// Algorithm discovery and capability enumeration.
pub mod discovery;

pub use asymmetric::*;
pub use discovery::*;
pub use hashing::*;
pub use symmetric::*;
