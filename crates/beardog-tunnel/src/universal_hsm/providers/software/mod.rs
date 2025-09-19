

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod attestation;
/// Configuration management
/// Configuration management
pub mod config;
/// Core functionality
/// Core functionality
pub mod core;
pub mod crypto;
pub mod entropy;
pub mod keystore;
pub mod memory;

pub use attestation::AttestationEngine;
pub use config::SoftwareHsmConfig;
pub use core::SoftwareHsmProvider;
pub use crypto::CryptoEngine;
pub use entropy::EntropyCollector;
pub use keystore::KeyStore;
pub use memory::SecureMemory;
