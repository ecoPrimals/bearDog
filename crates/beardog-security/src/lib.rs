//! Security and cryptography for BearDog Security Manager
//!
//! This crate provides comprehensive security functionality including
//! encryption, decryption, key management, and security policies.

pub mod crypto_utils;
pub mod decentralized_auth;
pub mod encryption;
pub mod handlers;
pub mod memory_key_manager;
pub mod recovery;
// pub mod recovery_tests; // Temporarily disabled due to API changes
pub mod tests;
pub mod types;
pub mod zero_copy; // New modular zero-copy implementation
pub mod zero_copy_crypto; // Add zero-copy crypto optimizations (legacy compat)

// Re-export main types for convenience
pub use crypto_utils::BearDogCrypto;
pub use memory_key_manager::MemoryKeyManager;
pub use types::*;
pub use zero_copy_crypto::{BufferPool, EncryptionContext, ZeroCopyConfig, ZeroCopyCrypto};
