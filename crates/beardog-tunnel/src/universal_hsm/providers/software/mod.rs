// SPDX-License-Identifier: AGPL-3.0-only



//! Software HSM provider implementation

pub mod attestation;
pub mod config;
pub mod core;
pub mod crypto;
pub mod entropy;
pub mod keystore;
pub mod memory;

// Re-exports
pub use attestation::SoftwareAttestation;
pub use config::SoftwareHsmConfig;
pub use core::SoftwareHsmProvider;
pub use crypto::SoftwareCryptoProvider;
pub use entropy::SoftwareEntropyCollector;
pub use keystore::SoftwareKeyStore;
pub use memory::SoftwareMemoryManager;
