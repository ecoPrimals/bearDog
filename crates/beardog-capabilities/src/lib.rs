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
//! use beardog_capabilities::{CapabilityRegistry, SecureTunnelProvider};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // 1. Create capability registry
//! let mut registry = CapabilityRegistry::new("beardog-instance-1");
//!
//! // 2. Register capabilities (generic traits, no primal names)
//! let tunnel_provider = MySecureTunnelImpl::new();
//! registry.register("secure_tunnel", tunnel_provider, metadata)?;
//!
//! // 3. Advertise capabilities
//! registry.advertise().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Example: Consumer Side (Any Primal)
//!
//! ```rust,no_run
//! use beardog_capabilities::{CapabilityDiscovery, SecureTunnelProvider};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // 1. Discover capabilities
//! let mut discovery = CapabilityDiscovery::new();
//! discovery.discover().await?;
//!
//! // 2. Get capability (provider identity unknown)
//! let tunnel_provider = discovery
//!     .find_capability::<dyn SecureTunnelProvider>("secure_tunnel")
//!     .await?;
//!
//! // 3. Use capability
//! let tunnel = tunnel_provider.establish_tunnel(peer).await?;
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
