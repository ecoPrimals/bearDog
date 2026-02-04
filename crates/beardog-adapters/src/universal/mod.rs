// Universal Adapters Module
//
// This module contains all universal adapters that replace hardcoded integrations
// with dynamic capability-based discovery and interaction.

pub mod adapter_types;
pub mod capability_based_adapter;
pub mod primal_capability_adapter;
pub mod primal_runtime_discovery;
pub mod capability_adapter;
pub mod extensible_adapter;
pub mod adapter_impl;
pub mod types;
pub mod capability_helpers;
pub mod capability_types;

pub mod entropy_capability_adapter;

pub mod vendor_adapter;

// Discovery client wiring (Deep Debt Principle #5)
pub mod beardog_discovery_client;

pub use adapter_types::*;
pub use capability_based_adapter::*;
pub use primal_capability_adapter::*;
pub use capability_adapter::*;
pub use extensible_adapter::*;
pub use adapter_impl::*;
pub use types::*;
pub use capability_helpers::*;
pub use capability_types::*;

/// Export entropy capability components
pub use entropy_capability_adapter::{
    UniversalEntropyCapabilityAdapter, EntropyCapabilityConfig,
    UniversalEntropyCapability, EntropyRequest, EntropyResponse,
    EntropyRateLimits, OwnershipRequirements,
};

/// Export discovery client (bridges beardog-discovery with beardog-adapters)
pub use beardog_discovery_client::BearDogDiscoveryClient;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorInfo {
    /// The code value
    pub code: String,
    /// The message value
    pub message: String,
    /// Optional details
    pub details: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests;
