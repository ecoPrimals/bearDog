// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM implementation types (DTOs)
//!
//! Lightweight, serializable identity types for software HSM and storage backends.
//! Cryptographic behavior lives in `beardog-tunnel` / platform HSM providers, not here.

use serde::{Deserialize, Serialize};

/// Rust-based software HSM implementation
///
/// A pure Rust HSM implementation that provides cryptographic operations
/// without requiring hardware security modules. Suitable for development,
/// testing, and environments where hardware HSMs are not available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustSoftwareHsm {
    /// Unique identifier for this HSM instance
    pub id: String,
}

impl RustSoftwareHsm {
    /// Creates a new Rust software HSM with the given ID
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    /// Creates a software HSM with a default ID
    #[must_use]
    pub fn with_default_id() -> Self {
        Self::new("rust-software-hsm")
    }
}

impl Default for RustSoftwareHsm {
    fn default() -> Self {
        Self::with_default_id()
    }
}

/// In-memory storage backend
///
/// A zero-sized type that represents in-memory key storage.
/// This is primarily used for testing and ephemeral key management.
/// Keys are not persisted and will be lost when the process terminates.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct InMemoryStorageBackend;

impl InMemoryStorageBackend {
    /// Creates a new in-memory storage backend
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_software_hsm_creation() {
        let hsm = RustSoftwareHsm::new("test-hsm");
        assert_eq!(hsm.id, "test-hsm");
    }

    #[test]
    fn test_rust_software_hsm_default() {
        let hsm = RustSoftwareHsm::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(hsm.id, "rust-software-hsm");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_in_memory_storage_backend_creation() {
        let backend = InMemoryStorageBackend::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        // Just ensure it compiles and creates
        let _ = backend;
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_in_memory_storage_backend_default() {
        let backend = InMemoryStorageBackend;
        let _ = backend;
    }
}
