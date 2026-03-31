// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Key Persistence and Public Key Management (Phase 2)
//!
//! Provides secure key storage, retrieval, and lifecycle management for cryptographic keys.
//! Supports both ephemeral keys and persistent storage (HSM-backed or file-based).

mod persistence;
mod store;
mod types;

#[cfg(test)]
#[path = "key_management_tests.rs"]
mod key_management_tests;

pub use store::KeyStore;
pub use types::{KeyMetadata, KeyStorage, KeyType, KeyUsage};
