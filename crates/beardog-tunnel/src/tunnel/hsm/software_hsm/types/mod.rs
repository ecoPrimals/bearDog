// SPDX-License-Identifier: AGPL-3.0-or-later

//! Software HSM types
//!
//! Definitions are split by concern: `storage`, `config`, `protected_memory`, `keys`,
//! `audit_logging`, `encryption`, `key_store`, and `health`.

mod storage;

mod config;
mod keys;
mod protected_memory;

mod audit_logging;
mod encryption;
mod health;
mod key_store;

#[cfg(test)]
mod tests;

// ✅ MIGRATED: Using real crypto providers from software_hsm/crypto_providers and canonical trait
pub use beardog_types::hsm::CryptoProvider; // Canonical trait
pub use beardog_types::hsm::InMemoryStorageBackend;

pub use crate::tunnel::hsm::software_hsm::audit::types::{
    AuditLogEntry, AuditLogFilter, OperationResult,
};

pub use audit_logging::{AuditLogger, AuditLoggerBackend};
pub use config::{EncryptionKey, MemoryProtector, SoftwareHsmConfig};
pub use encryption::{DefaultEncryptionKey, EncryptionKeyBackend};
pub use health::SoftwareHealthMonitor;
pub use key_store::SoftwareKeyStore;
pub use keys::SoftwareKey;
pub use protected_memory::{MemoryProtectorTrait, ProtectedMemory};
pub use storage::{
    FileStorageBackend, MemoryStorageBackend, SoftwareHsmStorageKind, StorageBackend,
};
