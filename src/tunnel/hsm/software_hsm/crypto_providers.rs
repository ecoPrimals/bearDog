//! # Software HSM Crypto Providers
//!
//! This module provides different cryptographic provider implementations for the Software HSM.
//! Each provider offers different cryptographic backends and capabilities.
//!
//! The implementation has been refactored into focused sub-modules:
//! - `rust_crypto`: RustCryptoProvider implementation
//! - `ring_crypto`: RingCryptoProvider implementation  
//! - `openssl_crypto`: OpenSslCryptoProvider implementation
//! - `factory`: Provider creation and capability functions
//!
//! ## Usage
//!
//! ```rust
//! use crate::tunnel::hsm::software_hsm::crypto_providers::*;
//!
//! // Create a crypto provider
//! let provider = create_crypto_provider(&CryptoBackend::Ring).await?;
//!
//! // Check capabilities
//! let capabilities = get_crypto_provider_capabilities(&CryptoBackend::Ring);
//! ```

pub mod crypto_providers;

// Re-export all public types and functions
pub use crypto_providers::*;
