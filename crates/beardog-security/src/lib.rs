//! Security and cryptography for BearDog Security Manager
//!
//! This crate provides comprehensive security functionality including
//! encryption, decryption, key management, and security policies.

pub mod crypto_utils;
pub mod encryption;
pub mod handlers;
pub mod memory_key_manager;
pub mod recovery;
pub mod recovery_tests;
pub mod tests;
pub mod types;

// Re-export commonly used types
pub use crypto_utils::*;
pub use encryption::*;
pub use memory_key_manager::*;
pub use recovery::*;
pub use types::*;
