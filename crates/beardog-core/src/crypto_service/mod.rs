//! # Protocol-Agnostic Crypto Service
//!
//! Core cryptographic service that can be exposed via any protocol
//! (HTTP, JSON-RPC, tarpc, etc.) without protocol-specific dependencies.
//!
//! ## Design Principles
//!
//! 1. **Protocol Neutrality**: No HTTP, RPC, or transport-specific types
//! 2. **Capability-Based**: Operations declared via capabilities
//! 3. **Sovereignty-Preserving**: No external dependencies
//! 4. **Zero-Copy Where Possible**: Efficient memory handling
//! 5. **Modern Idiomatic Rust**: Async, strong typing, zero unsafe
//!
//! ## Architecture
//!
//! ```text
//! CryptoService Trait (protocol-agnostic)
//!       ↓
//! BearDogCryptoService (implementation)
//!       ↓
//! Algorithm modules (AES, ChaCha, Ed25519, etc.)
//!       ↓
//! Exposed via HTTP, JSON-RPC, tarpc
//! ```
//!
//! ## Module Organization
//!
//! This module is organized for **maintainability and discoverability**:
//!
//! - [`r#trait`] - Protocol-agnostic trait definition
//! - [`types`] - Configuration and state types
//! - [`algorithms`](crate::crypto_service::algorithms) - Cryptographic algorithm implementations
//! - [`implementation`](crate::crypto_service::implementation) - BearDogCryptoService implementation
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use beardog_core::crypto_service::{BearDogCryptoService, CryptoService, CryptoServiceConfig};
//! use beardog_types::crypto_service::{CryptoAlgorithm, EncryptOptions};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create service with configuration
//! let config = CryptoServiceConfig {
//!     service_name: "my-app".to_string(),
//!     hsm_enabled: true,
//!     ..Default::default()
//! };
//! let service = BearDogCryptoService::new(config)?;
//!
//! // Encrypt data
//! let plaintext = b"Secret message";
//! let encrypted = service.encrypt(
//!     plaintext,
//!     CryptoAlgorithm::Aes256Gcm,
//!     EncryptOptions {
//!         key_id: "my-key".to_string(),
//!         ..Default::default()
//!     },
//! ).await?;
//!
//! // Algorithm discovery and capability checking
//! let capabilities = service.get_capabilities().await?;
//! println!("Supported algorithms: {:?}", capabilities.supported_algorithms);
//! # Ok(())
//! # }
//! ```

// Public modules
pub mod algorithms;
pub mod implementation;
pub mod r#trait;
pub mod types;

// Re-exports for ergonomic API
pub use implementation::BearDogCryptoService;
pub use r#trait::CryptoService;
pub use types::{CryptoServiceConfig, CryptoServiceState};

// Result type alias
/// Result type alias for crypto service operations.
///
/// Convenience type wrapping [`std::result::Result`] with [`beardog_errors::BearDogError`].
/// This allows shorter function signatures throughout the crypto service.
pub type Result<T> = std::result::Result<T, beardog_errors::BearDogError>;

// Test coverage expansion - December 17, 2025
#[cfg(test)]
mod tests_coverage_expansion_dec17;

// Deep edge case testing - December 18, 2025
#[cfg(test)]
mod tests_dec18_edge_cases;
