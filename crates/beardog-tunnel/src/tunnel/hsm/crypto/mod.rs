//! Universal Crypto Provider System
//!
//! Provides vendor-agnostic cryptographic operations through a trait-based
//! abstraction similar to our Universal HSM architecture.
//!
//! This module eliminates crypto library lock-in by allowing runtime discovery
//! and selection of crypto providers based on capabilities and requirements.

pub mod algorithms;
pub mod capabilities;
pub mod manager;
pub mod provider;
pub mod providers;
pub mod requirements;

// Re-exports for convenience
pub use algorithms::*;
pub use capabilities::*;
pub use manager::CryptoProviderManager;
pub use provider::UniversalCryptoProvider;
pub use providers::*;
pub use requirements::*;
