// SPDX-License-Identifier: AGPL-3.0-or-later

//! Protected key material in memory and the memory-protection trait boundary.

use beardog_errors::BearDogError;
use std::future::Future;

/// Protected memory wrapper
#[derive(Debug, Clone)]
pub struct ProtectedMemory {
    /// Encrypted data (shared via [`bytes::Bytes`] for cheap clones on hot paths)
    pub data: bytes::Bytes,
    /// Whether memory is protected
    pub protected: bool,
}

impl ProtectedMemory {
    /// Create new protected memory
    pub fn new(data: Vec<u8>, protected: bool) -> Self {
        Self {
            data: bytes::Bytes::from(data),
            protected,
        }
    }

    /// Wrap existing shared bytes (e.g. after [`bytes::Bytes::clone`] from the same buffer).
    pub fn from_bytes(data: bytes::Bytes, protected: bool) -> Self {
        Self { data, protected }
    }

    /// Get data reference
    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    /// Check if memory is protected
    pub const fn is_protected(&self) -> bool {
        self.protected
    }
}

/// Trait for memory protection operations
pub trait MemoryProtectorTrait: Send + Sync {
    /// Initialize memory protection
    fn initialize(&self) -> impl Future<Output = Result<(), BearDogError>> + Send;

    /// Protect key material in memory
    fn protect_key_material(
        &self,
        key_material: &[u8],
    ) -> impl Future<Output = Result<ProtectedMemory, BearDogError>> + Send;

    /// Unprotect key material
    fn unprotect_key_material(
        &self,
        protected: &ProtectedMemory,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Securely zeroize key material
    fn zeroize_key_material(
        &self,
        key_material: &mut [u8],
    ) -> impl Future<Output = Result<(), BearDogError>> + Send;
}
