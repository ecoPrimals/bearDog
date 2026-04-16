// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration structs for software HSM behavior (storage kind, memory protection flags, serde config).

use crate::tunnel::hsm::types::{KeyStorageType, MemoryProtectionLevel};
use serde::{Deserialize, Serialize};

use super::storage::SoftwareHsmStorageKind;

/// Memory protection configuration
#[derive(Debug, Clone)]
pub struct MemoryProtector {
    /// Use secure memory allocation
    pub secure_allocation: bool,
    /// Clear memory on deallocation
    pub clear_on_dealloc: bool,
    /// Lock memory to prevent swapping
    pub lock_memory: bool,
}

/// Encryption key configuration
#[derive(Debug, Clone)]
pub struct EncryptionKey {
    /// Key derivation method
    pub derivation_method: String,
    /// Key size in bits
    pub key_size: u32,
    /// Use hardware entropy
    pub hardware_entropy: bool,
}

/// Software HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {
    /// Storage backend configuration
    pub storage: SoftwareHsmStorageKind,
    /// Memory protection level
    pub memory_protection: MemoryProtectionLevel,
    /// Key storage type
    pub key_storage: KeyStorageType,
    /// Enable audit logging
    pub audit_logging: bool,
    /// Maximum number of keys (None = unlimited)
    pub max_keys: Option<usize>,
}
