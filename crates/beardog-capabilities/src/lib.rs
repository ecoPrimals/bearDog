//! # BearDog Capability Framework
//!
//! **"Primals know only themselves. Capabilities are discovered at runtime."**
//!
//! This crate provides a capability-based architecture for primal interaction,
//! ensuring BearDog maintains sovereignty while enabling any primal to discover
//! and use its cryptographic services.
//!
//! ## Core Principles
//!
//! 1. **Primal Sovereignty**: BearDog knows only itself
//! 2. **Runtime Discovery**: Capabilities discovered via mDNS/HTTP, not hardcoded
//! 3. **Zero Coupling**: No primal names in capability definitions
//! 4. **Extensibility**: New primals work without code changes
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────┐
//! │                 BearDog (Provider Side)                  │
//! ├──────────────────────────────────────────────────────────┤
//! │  1. Implements capability traits                         │
//! │  2. Registers with CapabilityRegistry                    │
//! │  3. Advertises via mDNS + HTTP                           │
//! └──────────────────────────────────────────────────────────┘
//!                          │
//!                          │ Advertisement
//!                          ▼
//! ┌──────────────────────────────────────────────────────────┐
//! │          Discovery Layer (mDNS + HTTP)                   │
//! └──────────────────────────────────────────────────────────┘
//!                          │
//!                          │ Discovery
//!                          ▼
//! ┌──────────────────────────────────────────────────────────┐
//! │             Any Primal (Consumer Side)                   │
//! ├──────────────────────────────────────────────────────────┤
//! │  1. CapabilityDiscovery finds providers                  │
//! │  2. Gets capability proxy                                │
//! │  3. Uses capability (provider identity unknown)          │
//! └──────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example: Provider Side (BearDog)
//!
//! ```rust,no_run
//! use beardog_capabilities::{CapabilityRegistry, CapabilityMetadata};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // 1. Create capability registry
//! let registry = CapabilityRegistry::new(
//!     "beardog-instance-1",
//!     "cryptographic_services",
//!     "http://localhost:8080"
//! );
//!
//! // 2. Define capability metadata
//! let metadata = CapabilityMetadata::new("secure_tunnel", "1.0")
//!     .with_description("Secure tunnel capability")
//!     .with_interface("SecureTunnelProvider")
//!     .with_endpoint("http://localhost:8080/capabilities/secure_tunnel");
//!
//! // 3. Register capabilities (generic traits, no primal names)
//! // Note: In real implementation, pass actual provider instance
//! registry.register("secure_tunnel", (), metadata);
//!
//! // 4. Advertise capabilities via mDNS/HTTP
//! registry.advertise().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Example: Consumer Side (Any Primal)
//!
//! ```rust,no_run
//! use beardog_capabilities::CapabilityMetadata;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // 1. Discover capabilities via HTTP (conceptual - uses external HTTP client)
//! // let response = http_client.get("http://discoverable-primal/capabilities").await?;
//! // let capabilities: Vec<CapabilityMetadata> = response.json().await?;
//!
//! // 2. Find desired capability (provider identity discovered at runtime)
//! let capabilities: Vec<CapabilityMetadata> = vec![
//!     CapabilityMetadata::new("secure_tunnel", "1.0")
//!         .with_endpoint("http://localhost:8080/capabilities/secure_tunnel")
//! ];
//!
//! let tunnel_capability = capabilities
//!     .iter()
//!     .find(|c| c.id == "secure_tunnel")
//!     .ok_or_else(|| beardog_errors::BearDogError::network(
//!         "secure_tunnel capability not found".to_string()
//!     ))?;
//!
//! // 3. Use capability endpoint (provider remains sovereign)
//! // Note: In real implementation, create client proxy from endpoint
//! let _endpoint = &tunnel_capability.endpoint;
//! # Ok(())
//! # }
//! ```
//!
//! ## Naming Convention
//!
//! **Note**: While capability IDs like "secure_tunnel" are generic, documentation
//! may reference specific protocols (e.g., "BTSP") for developer context. This is
//! acceptable as it aids human understanding while keeping the code architecture
//! fully sovereign and agnostic.

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod metadata;
pub mod registry;
pub mod traits;

// Re-export core types
pub use metadata::{CapabilityAdvertisement, CapabilityEndpoint, CapabilityMetadata};
pub use registry::CapabilityRegistry;
pub use traits::{
    BroadcastEncryptionProvider, KeyDerivationProvider, LineageSigningProvider,
    SecureTunnelProvider,
};

/// Result type for capability operations
pub type Result<T> = std::result::Result<T, beardog_errors::BearDogError>;
