// Universal Adapters Module
//
// This module contains all universal adapters that replace hardcoded integrations
// with dynamic capability-based discovery and interaction.

pub mod capability_based_adapter;
pub mod primal_capability_adapter;
pub mod capability_adapter;
pub mod extensible_adapter;
pub mod adapter_impl;
pub mod types;
pub mod capability_types;
pub mod service_registration;
pub mod commercial_extraction;
pub mod http_adapter;

pub mod entropy_capability_adapter;

pub mod vendor_adapter;
/// Zero-cost capability dispatch system - PERFORMANCE OPTIMIZED
pub mod zero_cost_capability_dispatch;

/// Modular performance benchmarking system - MODERNIZED
pub mod benchmarks;
/// Performance benchmarking system - DEPRECATED (use benchmarks instead)
pub mod performance_benchmarks;

pub use capability_based_adapter::*;
pub use primal_capability_adapter::*;
pub use capability_adapter::*;
pub use extensible_adapter::*;
pub use adapter_impl::*;
pub use types::*;
pub use capability_types::*;

/// Export entropy capability components
pub use entropy_capability_adapter::{
    UniversalEntropyCapabilityAdapter, EntropyCapabilityConfig,
    UniversalEntropyCapability, EntropyRequest, EntropyResponse,
    EntropyRateLimits, OwnershipRequirements,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorInfo {
    /// The code value
    pub code: String,
    /// The message value
    pub message: String,
    /// Optional details
    pub details: Option<serde_json::Value>,
}
