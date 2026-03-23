// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]

//! # beardog-discovery
//!
//! Capability-based service discovery for BearDog.
//!
//! ## Architecture Principles
//!
//! 1. **Self-Knowledge Only**: BearDog only knows what IT provides
//! 2. **Capability-Based**: Discover services by capability, not by name
//! 3. **Zero Hardcoding**: No hardcoded service names or endpoints
//! 4. **Runtime Discovery**: Find services at runtime via mDNS, DNS-SD, registries
//!
//! ## Example
//!
//! ```no_run
//! use beardog_discovery::{CapabilityDiscovery, DiscoveryConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Discover services that provide "orchestration" capability
//! let discovery = CapabilityDiscovery::from_config("configs/beardog-primal-capabilities.toml").await?;
//!
//! // Find ANY service that provides orchestration (don't care who!)
//! let orchestrators = discovery.find_by_capability("orchestration").await?;
//!
//! if let Some(service) = orchestrators.first() {
//!     println!("Found orchestrator at: {}", service.endpoint.primary_url);
//!     // Connect to service.endpoint (could be Songbird, or anything else!)
//! }
//! # Ok(())
//! # }
//! ```

pub mod announcement;
pub mod capability_env;
pub mod config;
pub mod discovery;
pub mod dns_sd;
pub mod error;
pub mod mdns;
pub mod service_registry;
pub mod types;

pub use announcement::{AnnouncementConfig, Announcer};
pub use capability_env::{
    DEFAULT_ENV_DISCOVERY_TTL_SECS, discovered_services_from_environment,
    discovered_services_from_environment_from_env, discovered_services_from_environment_with,
    primary_url_to_ipc_socket_path,
};
pub use config::DiscoveryConfig;
pub use discovery::{CapabilityDiscovery, EnvironmentDiscoveryFn};
pub use error::{DiscoveryError, Result};
pub use types::{Capability, DiscoveredService, PrimalInfo, ServiceEndpoint};

/// Re-exports for convenience
pub mod prelude {
    pub use crate::{
        announcement::{AnnouncementConfig, Announcer},
        config::DiscoveryConfig,
        discovery::CapabilityDiscovery,
        error::{DiscoveryError, Result},
        types::{Capability, DiscoveredService, PrimalInfo, ServiceEndpoint},
    };
}

#[cfg(test)]
mod coverage_boost_wave10;
