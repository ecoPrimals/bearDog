// SPDX-License-Identifier: AGPL-3.0-or-later

//! `BearDogCryptoService` implementation
//!
//! Orchestrates cryptographic operations by delegating to algorithm modules
//! and managing keys, state, and audit logging.

mod encrypt_decrypt;
mod keys;
mod lifecycle;
mod sign_verify;
mod trait_impl;

#[cfg(test)]
#[path = "implementation_tests.rs"]
mod implementation_tests;

use crate::crypto_service::algorithms::discovery;
use crate::crypto_service::types::{CryptoServiceConfig, CryptoServiceState};
use std::sync::Arc;

/// `BearDog` implementation of `CryptoService`
///
/// This implementation:
/// - Delegates to algorithm modules for actual crypto operations
/// - Manages key storage and derivation
/// - Provides audit logging
/// - Tracks operation metrics
/// - Discovers capabilities at runtime
pub struct BearDogCryptoService {
    /// Service configuration
    config: Arc<CryptoServiceConfig>,

    /// Runtime state (operation counters, uptime, etc.)
    state: Arc<CryptoServiceState>,

    /// Algorithm registry for capability discovery
    algorithms: discovery::AlgorithmRegistry,

    /// Public key storage for signature verification
    /// Maps `key_id` -> `public_key_bytes`
    ///
    /// # Security Note
    ///
    /// Only public keys are stored here - never private keys.
    /// This enables proper signature verification without exposing secrets.
    public_keys: Arc<parking_lot::RwLock<std::collections::HashMap<String, Vec<u8>>>>,

    /// RSA key storage (private keys)
    /// Maps `key_id` -> DER-encoded PKCS#8 private key
    ///
    /// # Security Note
    ///
    /// RSA private keys are stored in memory encrypted with AES-256-GCM.
    /// In production, these should be stored in HSM or secure key storage.
    /// This is an intermediate solution for development/testing.
    rsa_keys: Arc<parking_lot::RwLock<std::collections::HashMap<String, Vec<u8>>>>,
}
