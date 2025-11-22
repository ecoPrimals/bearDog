//! # Discovery Systems
//!
//! Provides various discovery mechanisms for BearDog, ranging from traditional hardcoded
//! systems to revolutionary infant discovery that starts with zero knowledge.
//!
//! ## Overview
//!
//! This module implements the "infant discovery pattern" where systems learn about their
//! environment dynamically without hardcoded assumptions. This eliminates the 2^n
//! hardcoding problem by using O(1) universal adapter patterns.
//!
//! ## Discovery Approaches
//!
//! - **Infant Discovery**: Zero-knowledge bootstrap, learns everything dynamically
//! - **Universal Discovery**: Vendor-agnostic service discovery
//! - **HSM Discovery**: Hardware Security Module detection and integration
//!
//! ## Key Components
//!
//! - [`infant_discovery`] - Revolutionary zero-knowledge discovery
//! - [`universal_infant_discovery`] - Universal service discovery patterns
//! - [`vendor_agnostic_hsm`] - Vendor-independent HSM discovery
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::discovery::infant_discovery::InfantDiscovery;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Start with zero knowledge
//! let discovery = InfantDiscovery::new();
//!
//! // Discover ecosystem capabilities dynamically
//! let capabilities = discovery.discover_capabilities().await?;
//! println!("Discovered {} services", capabilities.len());
//! # Ok(())
//! # }
//! ```

/// Infant discovery system - zero-knowledge bootstrap
pub mod infant_discovery;

/// Universal infant discovery patterns
pub mod universal_infant_discovery;

/// Vendor-agnostic HSM discovery and integration
pub mod vendor_agnostic_hsm;

pub use infant_discovery::*;
pub use universal_infant_discovery::*;
pub use vendor_agnostic_hsm::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod infant_discovery_tests;

#[cfg(test)]
mod universal_infant_discovery_tests;
