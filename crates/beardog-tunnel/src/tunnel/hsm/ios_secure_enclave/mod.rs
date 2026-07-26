// SPDX-License-Identifier: AGPL-3.0-or-later

//! iOS Secure Enclave HSM Module
//!
//! Provides safe, memory-safe-only access to iOS Secure Enclave functionality.
//!
//! ## Architecture
//!
//! - `safe_secure_enclave` — Production implementation using `security-framework`
//!   for real P-256 keygen and ECDSA signing on iOS. Fail-closed on non-iOS.
//!
//! ## Legacy modules (not yet evolved)
//!
//! `capability.rs`, `operations.rs`, and `types.rs` contain prototype code from
//! an earlier design phase. They are excluded from the module tree until they are
//! evolved to align with the canonical `HsmKeyProvider` trait and current type system.

pub mod safe_secure_enclave;

pub use safe_secure_enclave::SafeSecureEnclave;
