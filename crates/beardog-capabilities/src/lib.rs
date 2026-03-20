// SPDX-License-Identifier: AGPL-3.0-only

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
//! use beardog_capabilities::{CapabilityMetadata, CapabilityRegistry};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // 1. Create capability registry (HTTP/mDNS from BEARDOG_* env; see metadata module)
//! let registry = CapabilityRegistry::new(
//!     "550e8400-e29b-41d4-a716-446655440000",
//!     "cryptographic_services",
//! );
//!
//! // 2. Define capability metadata (endpoints come from discovery at runtime)
//! let metadata = CapabilityMetadata::new("secure_tunnel", "1.0")
//!     .with_description("Secure tunnel capability")
//!     .with_interface("SecureTunnelProvider")
//!     .with_endpoint("resolved-at-runtime-from-discovery");
//!
//! // 3. Register capabilities (generic traits, no coupled primal names)
//! // Note: In real implementation, pass actual provider instance
//! registry.register("secure_tunnel", (), metadata);
//!
//! // 4. Advertise capabilities via mDNS/HTTP
//! registry.advertise().await?;
//! # Ok(())
//! # }
//! ```
//!
//! To pin an HTTP base explicitly (integration tests, fixed config):
//! `CapabilityRegistry::with_http_base(instance_id, profile, "http://127.0.0.1:8080")`.
//!
//! ## Example: Consumer Side (Any Primal)
//!
//! ```rust,no_run
//! use beardog_capabilities::CapabilityMetadata;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // 1. Discover capabilities via HTTP / mDNS (external client; URLs not hardcoded)
//! // let response = http_client.get(&discovered_capability_url).await?;
//! // let capabilities: Vec<CapabilityMetadata> = response.json().await?;
//!
//! // 2. Find desired capability by id (provider instance id from discovery payload)
//! let capabilities: Vec<CapabilityMetadata> = vec![
//!     CapabilityMetadata::new("secure_tunnel", "1.0")
//!         .with_endpoint("https://provider-from-discovery/capabilities/secure_tunnel")
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

pub mod metadata;
pub mod registry;
pub mod traits;

// Re-export core types
pub use metadata::{
    CapabilityAdvertisement, CapabilityEndpoint, CapabilityMetadata, DEFAULT_CAPABILITY_HTTP_PATH,
    DEFAULT_CAPABILITY_MDNS_SERVICE_TYPE, DEFAULT_DISCOVERY_TTL_SECS, DiscoveryConfig,
    ENV_CAPABILITY_HTTP_BASE, ENV_CAPABILITY_HTTP_PATH, ENV_CAPABILITY_MDNS_INSTANCE,
    ENV_CAPABILITY_MDNS_SERVICE, PrimalInfo, resolve_capability_http_base,
    resolve_capability_http_discovery_url, resolve_capability_mdns_full_name,
};
pub use registry::CapabilityRegistry;
pub use traits::{
    BroadcastEncryptionProvider, KeyDerivationProvider, LineageSigningProvider,
    SecureTunnelProvider,
};

/// Result type for capability operations
pub type Result<T> = std::result::Result<T, beardog_errors::BearDogError>;
